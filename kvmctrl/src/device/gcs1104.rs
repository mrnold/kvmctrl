use log;

use crate::keyboard::create_keyboard;
use crate::keyboard::Keyboard;
use crate::keyboard::KeyPress;
use crate::keyboard::KeyCode;
use crate::SwitchController;
use crate::SwitchControl;
use crate::SwitchInput;

pub struct GCS1104 {
    keyboard: Keyboard,
}

impl GCS1104 {
    pub fn new() -> Result<Self, std::io::Error> {
        match create_keyboard() {
            Ok(keyboard) => return Ok(Self{keyboard: keyboard}),
            Err(error) => return Err(error),
        }
    }
}

impl SwitchController for GCS1104 {
    fn list_controls(&self) -> Vec<SwitchControl> {
        use SwitchInput::*;
        return vec![
            SwitchControl::AllInputsToNextPort,
            SwitchControl::AllInputsToPortNum(Port1),
            SwitchControl::AllInputsToPortNum(Port2),
            SwitchControl::AllInputsToPortNum(Port3),
            SwitchControl::AllInputsToPortNum(Port4),
            SwitchControl::KvmToNextPort,
        ];
    }

    fn issue_control(&self, control: SwitchControl) -> std::io::Result<() > {
        use KeyPress::*;
        use KeyCode::*;
        use SwitchInput::*;
        let sequence: Vec<KeyPress> = match control {
            SwitchControl::AllInputsToNextPort => vec![Tap(K), Tap(Enter)],
            SwitchControl::AllInputsToPortNum(Port1) => vec![Tap(ScrollLock), Tap(ScrollLock), Tap(One), Tap(Enter)],
            SwitchControl::AllInputsToPortNum(Port2) => vec![Tap(ScrollLock), Tap(ScrollLock), Tap(Two), Tap(Enter)],
            SwitchControl::AllInputsToPortNum(Port3) => vec![Tap(ScrollLock), Tap(ScrollLock), Tap(Three), Tap(Enter)],
            SwitchControl::AllInputsToPortNum(Port4) => vec![Tap(ScrollLock), Tap(ScrollLock), Tap(Four), Tap(Enter)],
            SwitchControl::KvmToNextPort => vec![Tap(ScrollLock), Tap(ScrollLock), Tap(K)],
            _ => return Ok(()),
        };
        for keypress in sequence {
            log::info!("Sending press: {:?}", keypress);
            self.keyboard.send_key(keypress);
        }
        Ok(())
    }
}
