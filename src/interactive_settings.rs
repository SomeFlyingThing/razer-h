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

fn parse_poll_rate(value: &str) -> Option<u16> {
    match value.trim().parse().ok()? {
        rate @ (8000 | 4000 | 2000 | 1000 | 500 | 250 | 125) => Some(rate),
        _ => None,
    }
}

fn parse_dpi(value: &str) -> Option<u16> {
    let dpi = value.trim().parse().ok()?;

    if dpi == 0 {
        return None;
    }

    Some(dpi)
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
                config.poll_rate = parse_poll_rate(value);
            }
            "dpi" => {
                config.dpi = parse_dpi(value);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_rate_parser_accepts_supported_values_up_to_8000() {
        for rate in ["125", "250", "500", "1000", "2000", "4000", "8000"] {
            assert_eq!(parse_poll_rate(rate), rate.parse().ok());
        }
    }

    #[test]
    fn poll_rate_parser_rejects_weird_or_impossible_values() {
        for rate in [
            "", "abc", "-1", "0", "1", "124", "126", "7999", "8001", "9999", "65536",
        ] {
            assert_eq!(parse_poll_rate(rate), None, "{rate} should be invalid");
        }
    }

    #[test]
    fn dpi_parser_rejects_non_numeric_out_of_range_and_zero_values() {
        for dpi in ["", "abc", "-1", "0", "65536"] {
            assert_eq!(parse_dpi(dpi), None, "{dpi} should be invalid");
        }
    }
}
