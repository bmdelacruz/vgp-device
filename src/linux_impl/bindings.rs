#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod consts {
    pub use super::{
        ABS_RX, ABS_RY, ABS_X, ABS_Y, BTN_A, BTN_B, BTN_DPAD_DOWN, BTN_DPAD_LEFT, BTN_DPAD_RIGHT,
        BTN_DPAD_UP, BTN_SELECT, BTN_START, BTN_THUMBL, BTN_THUMBR, BTN_TL, BTN_TL2, BTN_TR,
        BTN_TR2, BTN_X, BTN_Y, EV_ABS, EV_FF, EV_KEY, EV_SYN, EV_UINPUT, FF_MAX_EFFECTS, FF_RUMBLE,
        SYN_REPORT, UI_FF_ERASE, UI_FF_UPLOAD,
    };
}

pub mod types {
    pub use super::{
        input_event, uinput_abs_setup, uinput_ff_erase, uinput_ff_upload, uinput_setup,
    };
}
