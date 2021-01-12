#[derive(Debug)]
pub enum Button {
    DpadDown,
    DpadUp,
    DpadLeft,
    DpadRight,
    North,
    South,
    West,
    East,
    Start,
    Select,
    TriggerLeft,
    TriggerRight,
    TriggerLeft2,
    TriggerRight2,
    ThumbStickLeft,
    ThumbStickRight,
}

#[derive(Debug)]
pub enum ThumbStick {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Input {
    Press(Button),
    Release(Button),
    Move {
        thumb_stick: ThumbStick,
        x: f32,
        y: f32,
    },
}

#[derive(Debug)]
pub enum Output {
    None,
    Unsupported,
    Rumble { large_motor: u16, small_motor: u16 },
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub enum Error {
    PermissionDenied,
    Internal(nix::Error),
    Unknown(String),
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub enum Error {
    VigemBusNotInstalled,
    VigemBusVersionMismatch,
    Internal(vigem_client::Error),
    Unknown(String),
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub trait PlatformErrorExt<T> {
    fn map_with_vgp_error(self) -> Result<T, Error>;
}

#[cfg(target_os = "linux")]
impl<T> PlatformErrorExt<T> for Result<T, nix::Error> {
    fn map_with_vgp_error(self) -> Result<T, Error> {
        self.map_err(|e| match e {
            nix::Error::Sys(nix::errno::Errno::EACCES) => Error::PermissionDenied,
            _ => Error::Internal(e),
        })
    }
}

#[cfg(target_os = "windows")]
impl<T> PlatformErrorExt<T> for Result<T, vigem_client::Error> {
    fn map_with_vgp_error(self) -> Result<T, Error> {
        self.map_err(|e| match e {
            vigem_client::Error::BusNotFound => Error::VigemBusNotInstalled,
            vigem_client::Error::BusVersionMismatch => Error::VigemBusVersionMismatch,
            _ => Error::Internal(e),
        })
    }
}
