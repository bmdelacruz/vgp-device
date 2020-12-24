use std::{
    error::Error,
    fmt::{Display, Formatter},
};

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

#[derive(Debug)]
pub enum VgpDeviceButton {
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
pub enum VgpDeviceThumbStick {
    Left,
    Right,
}

#[derive(Debug)]
pub enum VgpDeviceInput {
    PressButton(VgpDeviceButton),
    ReleaseButton(VgpDeviceButton),
    MoveThumbStick {
        thumb_stick: VgpDeviceThumbStick,
        x: f32,
        y: f32,
    },
}

#[derive(Debug)]
pub enum VgpDeviceForceFeedbackType {
    Unsupported,
    Rumble {
        strong_magnitude: u16,
        weak_magnitude: u16,
    },
}

#[derive(Debug)]
pub struct VgpDeviceForceFeedbackReplay {
    pub length: u16,
    pub delay: u16,
}

#[derive(Debug)]
pub struct VgpDeviceForceFeedback {
    pub id: i16,
    pub direction: u16,
    pub replay: VgpDeviceForceFeedbackReplay,
    pub r#type: VgpDeviceForceFeedbackType,
}

#[derive(Debug)]
pub enum VgpDeviceEvent {
    None,
    Unsupported,
    ForceFeedbackPlayed(i16),
    ForceFeedbackStopped(i16),
    ForceFeedbackUploaded(VgpDeviceForceFeedback),
    ForceFeedbackErased(i16),
}

#[derive(Debug)]
pub struct VgpUnknownError {
    details: String,
}

impl VgpUnknownError {
    fn new(details: String) -> Self {
        Self { details }
    }
}

impl Display for VgpUnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for VgpUnknownError {}

#[derive(Debug)]
pub enum VgpError {
    PermissionDenied,
    Unknown(VgpUnknownError),
    #[cfg(target_os = "linux")]
    Internal(nix::Error),
    #[cfg(target_os = "windows")]
    Internal(Box<dyn Error>),
    #[cfg(target_os = "macos")]
    Internal(Box<dyn Error>),
}

impl Display for VgpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VgpError {}

pub trait VgpResultExt<T> {
    fn to_vgp_result(self) -> Result<T, VgpError>;
}

#[cfg(target_os = "linux")]
impl<T> VgpResultExt<T> for Result<T, nix::Error> {
    fn to_vgp_result(self) -> Result<T, VgpError> {
        self.map_err(|e| match e {
            nix::Error::Sys(nix::errno::Errno::EACCES) => VgpError::PermissionDenied,
            _ => VgpError::Internal(e),
        })
    }
}

pub trait VgpDevice: Sized + Send + Sync + Drop {
    fn new() -> Result<Self, VgpError>;
    fn make_input(&mut self, input: VgpDeviceInput) -> Result<(), VgpError>;
    fn get_next_event(&mut self) -> Result<VgpDeviceEvent, VgpError>;
}
