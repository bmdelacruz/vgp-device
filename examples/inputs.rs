use std::io::BufRead;

use vgp_device::VgpDevice;
use vgp_device::VgpDeviceButton;
use vgp_device::VgpDeviceImpl;
use vgp_device::VgpDeviceInput;

#[cfg(target_os = "linux")]
fn main() {
    let mut device = VgpDeviceImpl::new().unwrap();

    println!("device opened");

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        match line.as_str() {
            "x_press" => device
                .make_input(VgpDeviceInput::PressButton(VgpDeviceButton::North))
                .unwrap(),
            "x_release" => device
                .make_input(VgpDeviceInput::ReleaseButton(VgpDeviceButton::North))
                .unwrap(),
            "exit" => break,
            "" => {
                let ev = device.get_next_event().unwrap();
                println!("event: {:?}", ev);
            }
            _ => {}
        }
    }

    println!("device closed");
}

#[cfg(target_os = "windows")]
fn main() {
}
