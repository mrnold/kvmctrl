use log;

use usb_gadget::{default_udc, function, Class, Config, Gadget, Id, RegGadget, Strings};

#[derive(Debug)]
pub enum KeyCode {
    K = 0x0e,
    Enter = 0x28,
    One = 0x1e,
    Two = 0x1f,
    Three = 0x20,
    Four = 0x21,
    ScrollLock = 0x47,
}

#[derive(Debug)]
pub enum KeyPress {
    Up(KeyCode),
    Down(KeyCode),
    Tap(KeyCode),
}

#[derive(Debug)]
pub struct Keyboard {
    gadget: RegGadget,
    path: std::path::PathBuf,
}

impl Keyboard {
    pub fn keyboard_info(&self) {
        log::info!("Gadget name: {:?}", self.gadget.name())
    }

    pub fn send_key(&self, keypress: KeyPress) {
        use KeyPress::*;
        let mut reports = Vec::<[u8; 8]>::new();
        match keypress {
            Tap(k) => {reports.push([0u8, 0, k as u8, 0, 0, 0, 0, 0]);
                                reports.push([0u8, 0, 0, 0, 0 ,0 ,0, 0])},
            Down(k) => reports.push([0u8, 0, k as u8, 0, 0, 0, 0, 0]),
            Up(_) => reports.push([0u8, 0, 0, 0, 0, 0, 0, 0]),
        }
        for report in reports {
            for _ in 1..=20 { // Retry if not ready yet
                match std::fs::write(self.path.to_owned(), &report) {
                    Ok(_) => break,
                    Err(error) => {
                        log::warn!("Error on USB send: {})", error);
                        if error.to_string().contains("Cannot send after transport endpoint shutdown") {
                            log::warn!("Gadget may not be ready yet, retrying...");
                            std::thread::sleep(std::time::Duration::from_millis(250))
                        }
                    }
                }
            }
        }
    }
}

pub fn create_keyboard() -> Result<Keyboard, std::io::Error> {
    match usb_gadget::remove_all() {
        Ok(_) => log::info!("Removed existing USB gadgets."),
        Err(e) => {log::error!("Failed to remove existing gadgets!"); return Err(e)}
    };

    let class = Class::new(0x00, 0x00, 0x00);
    let id = Id::new(0x0104, 0x1d6b); // Linux Foundation, multifunction composite gadget
    let strings = Strings::new("github.com/mrnold", "USB gadget KVM switch controller", "0000000001");

    let mut builder = function::hid::Hid::builder();
    builder.protocol = 0x01;
    builder.sub_class = 0x01;
    builder.report_len = 0x08;
    builder.report_desc = vec![
        0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x05, 0x07,
        0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00, 0x25, 0x01,
        0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01,
        0x75, 0x08, 0x81, 0x03, 0x95, 0x05, 0x75, 0x01,
        0x05, 0x08, 0x19, 0x01, 0x29, 0x05, 0x91, 0x02,
        0x95, 0x01, 0x75, 0x03, 0x91, 0x03, 0x95, 0x06,
        0x75, 0x08, 0x15, 0x00, 0x25, 0x65, 0x05, 0x07,
        0x19, 0x00, 0x29, 0x65, 0x81, 0x00, 0xc0,
    ];
    let (hid, handle ) = builder.build();

    let mut config = Config::new("HID gadget");
    config.add_function(handle);

    let mut gadget = Gadget::new(class, id, strings);
    gadget.add_config(config);

    let udc = match default_udc() {
        Ok(udc) => {log::debug!("Default UDC: {udc:?}\n"); udc},
        Err(error) => return Err(error),
    };

    log::debug!("Gadget: {gadget:?}\n");
    let registered_gadget = match gadget.bind(&udc) {
        Ok(gadget) => {log::info!("Registered gadget: {gadget:?}\n"); gadget},
        Err(error) => {log::error!("Failed to register gadget: {error:?}\n"); return Err(error)},
    };

    let minor = match hid.device() {
        Ok((major, minor)) => {log::debug!("Device numbers: {} {}", major, minor); minor},
        Err(error) => {log::error!("Failed to get HID!"); return Err(error)},
    };

    let mut path = std::path::PathBuf::new();
    path.push(std::format!("/dev/hidg{}", minor));

    let keyboard = Keyboard {
        gadget: registered_gadget,
        path: path,
    };
    return Ok(keyboard);
}