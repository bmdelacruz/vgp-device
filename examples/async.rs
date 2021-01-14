#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{stdin, BufRead};
    use tokio::sync::mpsc::unbounded_channel;
    use vgp_device::{Bus, Button, Input};

    simple_logger::SimpleLogger::default().init().unwrap();

    let mut bus = Bus::new().unwrap();
    let mut device = bus.plug_in().unwrap();

    log::info!(
        "device opened. enter a command and then press enter \
        to do something. available commands: `press-x`, \
        `release-x`, `exit`."
    );

    let (tx, mut rx) = unbounded_channel::<Input>();

    let join_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                input = rx.recv() => match input {
                    None => {
                        log::info!("No more input will be sent.");
                        break;
                    },
                    Some(input) => match device.put_input(input) {
                        Ok(_) => {
                            log::info!("Successfully put input to device.");
                        }
                        Err(e) => {
                            log::error!("An error occurred while putting input to device. {:?}", e);
                            break;
                        }
                    },
                },
                output = device.get_output() => match output {
                    Some(output) => {
                        log::info!("Received an output: {:?}", output);
                    }
                    None => {
                        log::info!("No more output will be received.");
                        break;
                    },
                },
            }
        }

        if let Err(e) = device.unplug() {
            log::error!("device closed with an error: {:?}", e);
        } else {
            log::info!("device closed");
        }
    });

    for line in stdin().lock().lines() {
        if tx.is_closed() {
            break;
        }
        match line {
            Ok(line) => match line.as_str() {
                "press-x" => tx.send(Input::Press(Button::West)).unwrap(),
                "release-x" => tx.send(Input::Release(Button::West)).unwrap(),
                "exit" => break,
                _ => {}
            },
            Err(_) => break,
        }
    }

    std::mem::drop(tx);

    join_handle.await.unwrap();

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    panic!("The feature `async` should be enabled to run this example.");
}
