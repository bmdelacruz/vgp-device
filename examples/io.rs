use std::io::BufRead;

use vgp_device::{Bus, Button, Input};

fn main() {
    simple_logger::SimpleLogger::default().init().unwrap();

    let mut bus = Bus::new().unwrap();
    let mut device = bus.plug_in().unwrap();

    log::info!("device opened");

    for line in std::io::stdin().lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        match line.as_str() {
            "x_press" => device.put_input(Input::Press(Button::North)).unwrap(),
            "x_release" => device.put_input(Input::Release(Button::North)).unwrap(),
            "exit" => break,
            "" => {
                let ev = device.get_output().unwrap();
                log::info!("event: {:?}", ev);
            }
            _ => {}
        }
    }

    device.unplug().unwrap();

    log::info!("device closed");
}
