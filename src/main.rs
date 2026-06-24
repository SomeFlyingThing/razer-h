use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use crate::interactive_settings::REST;

const RAZER_CONTROL_INTERFACE: u8 = 2;

mod interactive_settings;
mod protocol;
mod setulp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        create_config_if_missing();

        let device = setulp::get_device();
        if device.is_none() {
            eprintln!("No supported Razer device found.");
            return;
        }
        let device = match device.unwrap().open() {
            Ok(device) => device,
            Err(error) => {
                eprintln!("Failed to open the Razer device: {error}");
                return;
            }
        };

        if let Err(error) = device.set_auto_detach_kernel_driver(true) {
            eprintln!("Failed to enable automatic HID driver detachment: {error}");
            return;
        }

        if let Err(error) = device.claim_interface(RAZER_CONTROL_INTERFACE) {
            eprintln!("Failed to claim Razer interface {RAZER_CONTROL_INTERFACE}: {error}");
            return;
        }

        if let Err(error) = interactive_settings::set_interactive_settings(&device) {
            eprintln!("Failed to configure the Razer device: {error}");
        }

        if let Err(error) = device.release_interface(RAZER_CONTROL_INTERFACE) {
            eprintln!("Failed to release Razer interface {RAZER_CONTROL_INTERFACE}: {error}");
        }
    } else {
        print!("args aren't supported for now");
    }
}

fn create_config_if_missing() {
    let home = env::var("HOME").expect("HOME is not set");
    let path = PathBuf::from(home).join(REST.trim_start_matches('/'));

    if path.exists() {
        return;
    }

    let mut poll_rate = String::new();
    print!("Polling rate: ");
    io::stdout().flush().expect("couldn't flush stdout");
    io::stdin()
        .read_line(&mut poll_rate)
        .expect("couldn't read polling rate");

    let mut dpi = String::new();
    print!("DPI: ");
    io::stdout().flush().expect("couldn't flush stdout");
    io::stdin().read_line(&mut dpi).expect("couldn't read DPI");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("couldn't create config directory");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("couldn't create config file");

    writeln!(file, "dpi={}", dpi.trim()).expect("couldn't write config file");
    writeln!(file, "poll_rate={}", poll_rate.trim()).expect("couldn't write config file");
}
