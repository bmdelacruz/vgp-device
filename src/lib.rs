mod api;
pub use api::*;

#[cfg(target_os = "linux")]
mod linux_impl;
#[cfg(target_os = "linux")]
pub use linux_impl::*;

#[cfg(target_os = "windows")]
mod windows_impl;
#[cfg(target_os = "windows")]
pub use windows_impl::*;

#[cfg(target_os = "macos")]
mod macos_impl;
#[cfg(target_os = "macos")]
pub use macos_impl::*;

mod type_tests {
    #![allow(dead_code)]
    #![allow(unused_imports)]

    use super::*;

    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    #[test]
    fn bus_and_device_is_send() {
        is_send::<Bus>();
        is_send::<Device>();
    }

    #[test]
    fn bus_and_device_is_sync() {
        is_sync::<Bus>();
        is_sync::<Device>();
    }
}
