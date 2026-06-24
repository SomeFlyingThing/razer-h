use crate::protocol;
use rusb::{DeviceHandle, GlobalContext};
use std::{env, fs::OpenOptions, io::Read};

pub static REST: &str = "/.config/razer-h";

pub fn set_interactive_settings(handle: &DeviceHandle<GlobalContext>) -> rusb::Result<()> {
    let mut info = Info {
        dpi: None,
        poll_rate: None,
    };

    verify_n_read_file(&mut info);

    if info.dpi.is_none() {
        info.dpi = Some(1600);
    }

    if info.poll_rate.is_none() {
        info.poll_rate = Some(8000);
    }

    protocol::set_dpi_settings(info.dpi.unwrap(), handle)?;
    protocol::set_onboard_polling(info.poll_rate.unwrap(), handle)
}

pub struct Info {
    pub poll_rate: Option<u16>,
    pub dpi: Option<u16>,
}

fn verify_n_read_file(config: &mut Info) {
    let home = env::var("HOME").unwrap();

    let mut final_ = home;
    final_.push_str(REST);

    let file = OpenOptions::new().read(true).create(false).open(final_);

    let mut file = match file {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let Some((option, value)) = line.split_once("=") else {
            continue;
        };

        match option {
            "poll_rate" => {
                config.poll_rate = value.trim().parse().ok();
            }
            "dpi" => {
                config.dpi = value.trim().parse().ok();
            }
            _ => {}
        }
    }
}
