use rusb::{DeviceHandle, GlobalContext};
use std::io;

use crate::protocol;

pub fn set_interactive_settings(handle: &DeviceHandle<GlobalContext>) {
    let mut answer = String::new();
    print!("what dpi do you want followed by the number for polling rate?");

    io::stdin().read_line(&mut answer).unwrap();

    answer = answer.trim().to_string();

    //vector to store the choices
    let mut choices = vec![];

    //colletct the args
    for word in answer.split_whitespace() {
        choices.push(word);
    }

    let dpi: u16 = choices[0].parse().unwrap();
    let poll: u16 = choices[1].parse().unwrap();

    protocol::set_dpi_settings(dpi, handle);
    protocol::set_onboard_polling(poll, handle);
}
