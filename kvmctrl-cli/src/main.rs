use clap::Parser;

mod command;
use command::{Command, Subcommands};

use kvmctrl::{self, SwitchControl, SwitchController, SwitchInput};

fn main() {
    let run = Command::parse();

    match run.debug {
        1 => simple_logger:: init_with_level(log::Level::Debug).unwrap(),
        2 => simple_logger:: init_with_level(log::Level::Trace).unwrap(),
        _ => simple_logger:: init_with_level(log::Level::Info).unwrap(),
    }

    let portnum: u8 = match &run.command {
        Some(Subcommands::All(args)) => {
            log::info!("Switching everything to port number {}", args.port);
            args.port
        },
        None => {log::info!("Couldn't get port number from command line!"); return; }
    };

    let switcher = match kvmctrl::create_kvmctrl(kvmctrl::SwitchDevice::GCS1104) {
        Ok(switcher) => switcher,
        Err(error) => {log::error!("Failed to create KVM switch gadget: {}", error); return},
    };

    let port: SwitchInput = match portnum.try_into() {
        Ok(port) => port,
        Err(error) => {log::error!("Bad port number {}: {}", portnum, error); return}
    };

    match switcher.issue_control(SwitchControl::AllInputsToPortNum(port)) {
        Ok(_) => log::info!("Switched console to port {}", port),
        Err(error) => log::error!("Failed to switch console to port {}, {}", portnum, error),
    };
}