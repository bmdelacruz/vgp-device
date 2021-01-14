#[cfg(not(feature = "async"))]
fn main() {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use vgp_device::Bus;

    simple_logger::SimpleLogger::default().init().unwrap();

    let mut bus = Bus::new().unwrap();
    let device = bus.plug_in().unwrap();

    log::info!("device opened. press ctrl-c to close it.");

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);

    ctrlc::set_handler(move || {
        should_stop_clone.store(true, Ordering::SeqCst);
    })
    .unwrap();

    while !should_stop.load(Ordering::SeqCst) {}

    device.unplug().unwrap();

    log::info!("device closed");
}

#[cfg(feature = "async")]
fn main() {
    panic!("The feature `async` should be disabled to run this example.");
}
