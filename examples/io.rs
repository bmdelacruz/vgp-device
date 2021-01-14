#[cfg(not(feature = "async"))]
fn main() {
    use std::io::BufRead;
    use vgp_device::{Bus, Button, Input};

    simple_logger::SimpleLogger::default().init().unwrap();

    let mut bus = Bus::new().unwrap();
    let mut device = bus.plug_in().unwrap();

    log::info!(
        "device opened. enter a command and then press enter \
        to do something. available commands: `press-x`, \
        `release-x`, `exit`. Just press enter to get output \
        from the device. Please take note that outputs don't \
        immediately appear; you may need to press enter \
        multiple times."
    );

    for line in std::io::stdin().lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        match line.as_str() {
            "press-x" => device.put_input(Input::Press(Button::North)).unwrap(),
            "release-x" => device.put_input(Input::Release(Button::North)).unwrap(),
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

#[cfg(feature = "async")]
fn main() {
    panic!("The feature `async` should be disabled to run this example.");
}
