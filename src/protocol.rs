pub struct RazerReport {
    pub status: u8,
    pub transaction_id: u8,
    pub remaining_packets: u16,

    pub protocol_type: u8,

    pub data_size: u8,
    pub command_class: u8,
    pub command_id: u8,

    pub arguments: [u8; 80],

    pub crc: u8,
    pub reserved: u8,
}

fn make_set_dpi_report(dpi: u16) -> RazerReport {
    let mut report = RazerReport {
        status: 0x00,
        transaction_id: 0x3f,
        remaining_packets: 0x0000,
        protocol_type: 0x00,

        data_size: 0x07,
        command_class: 0x04,
        command_id: 0x05,

        arguments: [0u8; 80],

        crc: 0x00,
        reserved: 0x00,
    };

    let [hi, lo] = dpi.to_be_bytes();

    report.arguments[0] = 0x00; // first DPI stage
    report.arguments[1] = hi; // X DPI high byte
    report.arguments[2] = lo; // X DPI low byte
    report.arguments[3] = hi; // Y DPI high byte
    report.arguments[4] = lo; // Y DPI low byte
    report.arguments[5] = 0x00;
    report.arguments[6] = 0x00;

    report
}

use rusb::{DeviceHandle, GlobalContext};
use std::time::Duration;

fn send_report(handle: &DeviceHandle<GlobalContext>, report: &mut RazerReport) -> rusb::Result<()> {
    let mut bytes = [0u8; 90];

    bytes[0] = report.status;
    bytes[1] = report.transaction_id;
    bytes[2..4].copy_from_slice(&report.remaining_packets.to_be_bytes());
    bytes[4] = report.protocol_type;
    bytes[5] = report.data_size;
    bytes[6] = report.command_class;
    bytes[7] = report.command_id;
    bytes[8..88].copy_from_slice(&report.arguments);

    // compute XOR checksum
    let mut crc = 0u8;

    for b in &bytes[2..88] {
        crc ^= *b;
    }

    report.crc = crc;
    bytes[88] = report.crc;
    bytes[89] = report.reserved;

    // HID SET_REPORT
    handle.write_control(
        0x21,   // request type
        0x09,   // HID_REQ_SET_REPORT
        0x0300, // value
        0x0002, // HID interface index used by most Razer devices
        &bytes,
        Duration::from_secs(1),
    )?;

    Ok(())
}

fn make_set_polling_report(rate: u16, arg0: u8) -> RazerReport {
    let code = match rate {
        8000 => 0x01,
        4000 => 0x02,
        2000 => 0x04,
        1000 => 0x08,
        500 => 0x10,
        250 => 0x20,
        125 => 0x40,
        _ => 0x10, // default 500
    };

    let mut report = RazerReport {
        status: 0x00,
        transaction_id: 0x1f,
        remaining_packets: 0x0000,
        protocol_type: 0x00,

        data_size: 0x02,
        command_class: 0x00,
        command_id: 0x40,

        arguments: [0u8; 80],
        crc: 0x00,
        reserved: 0x00,
    };

    report.arguments[0] = arg0;
    report.arguments[1] = code;

    report
}

pub fn set_onboard_polling(poll: u16, handle: &DeviceHandle<GlobalContext>) -> rusb::Result<()> {
    let mut report = make_set_polling_report(poll, 0x00);

    send_report(handle, &mut report)?;

    let mut report = make_set_polling_report(poll, 0x01);

    send_report(handle, &mut report)
}

pub fn set_dpi_settings(dpi: u16, handle: &DeviceHandle<GlobalContext>) -> rusb::Result<()> {
    let mut report = make_set_dpi_report(dpi);

    send_report(handle, &mut report)
}
