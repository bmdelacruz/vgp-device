use std::sync::{atomic::AtomicBool, Arc};

use vgp_device::Bus;

fn main() {
    simple_logger::SimpleLogger::default().init().unwrap();

    let mut bus = Bus::new().unwrap();
    let device = bus.plug_in().unwrap();

    log::info!("device opened");

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);

    ctrlc::set_handler(move || {
        should_stop_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    while !should_stop.load(std::sync::atomic::Ordering::SeqCst) {}

    device.unplug().unwrap();

    log::info!("device closed");
}
