use rusb::{Device, GlobalContext};

const RAZER_VENDOR_ID: u16 = 0x1532;

const RAZER_PRODUCT_ID_WIRED: u16 = 0x00c0;
const RAZER_PRODUCT_ID_WIRELESS: u16 = 0x00c1;

pub fn get_device() -> Option<Device<GlobalContext>> {
    let context = rusb::devices().unwrap();
    for device in context.iter() {
        let desc = match device.device_descriptor() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if desc.vendor_id() != RAZER_VENDOR_ID {
            continue;
        }

        if !matches!(
            desc.product_id(),
            RAZER_PRODUCT_ID_WIRED | RAZER_PRODUCT_ID_WIRELESS
        ) {
            continue;
        }
        return Some(device);
    }
    None
}
