pub mod keyboard;

#[derive(Clone, Copy)]
pub enum SwitchInput { // Input ports
    Port1,
    Port2,
    Port3,
    Port4,
}

impl std::fmt::Display for SwitchInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SwitchInput::*;
        match self {
            Port1 => write!(f, "1"),
            Port2 => write!(f, "2"),
            Port3 => write!(f, "3"),
            Port4 => write!(f, "4"),
        }
    }
}

impl TryFrom<u8> for SwitchInput {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use SwitchInput::*;
        match value {
            1 => Ok(Port1),
            2 => Ok(Port2),
            3 => Ok(Port3),
            4 => Ok(Port4),
            _ => Err("Port number not in range 1-4.")
        }
    }
}

pub enum SwitchControl { // Actions the switch is able to do
    AllInputsToNextPort, // KVM, USB, and audio
    AllInputsToPortNum(SwitchInput),
    KvmToNextPort, // Just KVM
    KvmToPortNum,
    UsbToNextPort, // Just USB
    UsbToPortNum,
    AudioToNextPort, // Audio only
    AudioToPortNum,
    KvmAndAudioToPortNum, // KVM and audio
    KvmAndUsbToPortNum, // KVM and USB
    UsbAndAudioToPortNum, // USB and audio
    AutoScanDefaultInterval, // Cycle ports every five seconds
    AutoScanIntervalSeconds, // Cycle ports every N seconds
}

pub trait SwitchController {
    fn list_controls(&self) -> Vec<SwitchControl>; // Get a list of things this KVM switch can do
    fn issue_control(&self, control: SwitchControl) -> std::io::Result<()>;
}

mod device;
use core::fmt;

use device::GCS1104;
pub enum SwitchDevice {
    GCS1104,
}


pub fn create_kvmctrl(switch: SwitchDevice) -> Result<impl SwitchController, std::io::Error> {
    match switch {
        SwitchDevice::GCS1104 => GCS1104::new()
    }
}