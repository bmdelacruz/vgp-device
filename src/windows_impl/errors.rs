use super::bindings::*;

#[derive(Debug)]
pub enum Error {
    BusNotFound,
    NoFreeSlot,
    InvalidTarget,
    RemovalFailed,
    AlreadyConnected,
    TargetUninitialized,
    TargetNotPluggedIn,
    BusVersionMismatch,
    BusAccessFailed,
    CallbackAlreadyRegistered,
    CallbackNotFound,
    BusAlreadyConnected,
    BusInvalidHandle,
    XUSBUserIndexOutOfRange,
    InvalidParameter,
    NotSupported,
    PlugInError(Box<Error>, Box<Error>),
    Unknown,
}

pub trait ClientErrorConvertable {
    fn to_error(&self) -> Option<Error>;
}

impl ClientErrorConvertable for VIGEM_ERROR {
    fn to_error(&self) -> Option<Error> {
        match *self {
            _VIGEM_ERRORS_VIGEM_ERROR_NONE => None,
            _VIGEM_ERRORS_VIGEM_ERROR_BUS_NOT_FOUND => Some(Error::BusNotFound),
            _VIGEM_ERRORS_VIGEM_ERROR_NO_FREE_SLOT => Some(Error::NoFreeSlot),
            _VIGEM_ERRORS_VIGEM_ERROR_INVALID_TARGET => Some(Error::InvalidTarget),
            _VIGEM_ERRORS_VIGEM_ERROR_REMOVAL_FAILED => Some(Error::RemovalFailed),
            _VIGEM_ERRORS_VIGEM_ERROR_ALREADY_CONNECTED => Some(Error::AlreadyConnected),
            _VIGEM_ERRORS_VIGEM_ERROR_TARGET_UNINITIALIZED => Some(Error::TargetUninitialized),
            _VIGEM_ERRORS_VIGEM_ERROR_TARGET_NOT_PLUGGED_IN => Some(Error::TargetNotPluggedIn),
            _VIGEM_ERRORS_VIGEM_ERROR_BUS_VERSION_MISMATCH => Some(Error::BusVersionMismatch),
            _VIGEM_ERRORS_VIGEM_ERROR_BUS_ACCESS_FAILED => Some(Error::BusAccessFailed),
            _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_ALREADY_REGISTERED => {
                Some(Error::CallbackAlreadyRegistered)
            }
            _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_NOT_FOUND => Some(Error::CallbackNotFound),
            _VIGEM_ERRORS_VIGEM_ERROR_BUS_ALREADY_CONNECTED => Some(Error::BusAlreadyConnected),
            _VIGEM_ERRORS_VIGEM_ERROR_BUS_INVALID_HANDLE => Some(Error::BusInvalidHandle),
            _VIGEM_ERRORS_VIGEM_ERROR_XUSB_USERINDEX_OUT_OF_RANGE => {
                Some(Error::XUSBUserIndexOutOfRange)
            }
            _VIGEM_ERRORS_VIGEM_ERROR_INVALID_PARAMETER => Some(Error::InvalidParameter),
            _VIGEM_ERRORS_VIGEM_ERROR_NOT_SUPPORTED => Some(Error::NotSupported),
            _ => Some(Error::Unknown),
        }
    }
}
