use std::sync::{atomic::AtomicBool, Arc};

use vgp_device::VgpDevice;
use vgp_device::VgpDeviceImpl;

#[cfg(target_os = "linux")]
fn main() {
    let _device = VgpDeviceImpl::new().unwrap();

    println!("device opened");

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);

    ctrlc::set_handler(move || {
        should_stop_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    while !should_stop.load(std::sync::atomic::Ordering::SeqCst) {}

    println!("device closed");
}
