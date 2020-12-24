use std::alloc::{alloc, dealloc, Layout};

use super::bindings::consts::*;
use super::bindings::types::*;
use crate::{VgpDeviceForceFeedback, VgpDeviceForceFeedbackReplay, VgpDeviceForceFeedbackType};

pub struct SafeSetup {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
    pub ff_effects_max: u32,
    pub name: &'static str,
}

pub struct Setup {
    layout: Layout,
    raw_ptr: *mut u8,
}

impl Setup {
    pub fn new() -> Setup {
        let layout = Layout::new::<uinput_setup>();
        let raw_ptr = unsafe { alloc(layout) };

        Self { layout, raw_ptr }
    }

    pub fn const_ptr(&self) -> *const uinput_setup {
        self.raw_ptr as *const uinput_setup
    }
}

impl From<SafeSetup> for Setup {
    fn from(safe: SafeSetup) -> Self {
        let setup = Self::new();
        let setup_ptr = setup.raw_ptr as *mut uinput_setup;

        unsafe {
            (*setup_ptr).id.bustype = safe.bustype;
            (*setup_ptr).id.vendor = safe.vendor;
            (*setup_ptr).id.product = safe.product;
            (*setup_ptr).id.version = safe.version;
            (*setup_ptr).ff_effects_max = safe.ff_effects_max;

            libc::strcpy(
                (*setup_ptr).name.as_mut_ptr(),
                safe.name.as_ptr() as *const i8,
            );
        }

        setup
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.raw_ptr, self.layout);
        }
    }
}

pub struct SafeAbsSetup {
    pub code: u16,
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}

pub struct AbsSetup {
    layout: Layout,
    raw_ptr: *mut u8,
}

impl AbsSetup {
    pub fn new() -> AbsSetup {
        let layout = Layout::new::<uinput_abs_setup>();
        let raw_ptr = unsafe { alloc(layout) };

        Self { layout, raw_ptr }
    }

    pub fn const_ptr(&self) -> *const uinput_abs_setup {
        self.raw_ptr as *const uinput_abs_setup
    }
}

impl From<SafeAbsSetup> for AbsSetup {
    fn from(safe: SafeAbsSetup) -> Self {
        let setup = Self::new();
        let setup_ptr = setup.raw_ptr as *mut uinput_abs_setup;

        unsafe {
            (*setup_ptr).code = safe.code;
            (*setup_ptr).absinfo.value = safe.value;
            (*setup_ptr).absinfo.minimum = safe.minimum;
            (*setup_ptr).absinfo.maximum = safe.maximum;
            (*setup_ptr).absinfo.fuzz = safe.fuzz;
            (*setup_ptr).absinfo.flat = safe.flat;
            (*setup_ptr).absinfo.resolution = safe.resolution;
        }

        setup
    }
}

impl Drop for AbsSetup {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.raw_ptr, self.layout);
        }
    }
}

#[derive(Debug)]
pub struct SafeInputEvent {
    pub code: u16,
    pub r#type: u16,
    pub value: i32,
    pub time: (i64, i64),
}

impl From<InputEvent> for SafeInputEvent {
    fn from(event: InputEvent) -> Self {
        let ptr = unsafe { *(event.raw_ptr as *const input_event) };
        Self {
            code: ptr.code,
            r#type: ptr.type_,
            value: ptr.value,
            time: (ptr.time.tv_sec, ptr.time.tv_usec),
        }
    }
}

pub struct InputEvent {
    layout: Layout,
    raw_ptr: *mut u8,
}

impl InputEvent {
    pub fn new() -> InputEvent {
        let layout = Layout::new::<input_event>();
        let raw_ptr = unsafe { alloc(layout) };

        Self { layout, raw_ptr }
    }

    pub fn size() -> usize {
        std::mem::size_of::<input_event>()
    }

    pub fn c_ptr(&self) -> *mut libc::c_void {
        self.raw_ptr as *mut libc::c_void
    }
}

impl From<SafeInputEvent> for InputEvent {
    fn from(safe: SafeInputEvent) -> Self {
        let event = Self::new();
        let event_ptr = event.raw_ptr as *mut input_event;

        unsafe {
            (*event_ptr).time.tv_sec = safe.time.0;
            (*event_ptr).time.tv_usec = safe.time.1;
            (*event_ptr).type_ = safe.r#type;
            (*event_ptr).code = safe.code;
            (*event_ptr).value = safe.value;
        }

        event
    }
}

impl Drop for InputEvent {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.raw_ptr, self.layout);
        }
    }
}

pub struct ForceFeedbackUpload {
    layout: Layout,
    raw_ptr: *mut u8,
}

impl ForceFeedbackUpload {
    pub fn new(request_id: u32) -> Self {
        let layout = Layout::new::<uinput_ff_upload>();
        let raw_ptr = unsafe { alloc(layout) };

        unsafe {
            (*(raw_ptr as *mut uinput_ff_upload)).request_id = request_id;
        }

        Self { layout, raw_ptr }
    }

    pub fn mut_ptr(&self) -> *mut uinput_ff_upload {
        self.raw_ptr as *mut uinput_ff_upload
    }

    pub fn set_retval(&mut self, retval: i32) {
        unsafe {
            (*(self.raw_ptr as *mut uinput_ff_upload)).retval = retval;
        }
    }

    pub fn to_vgp_device_force_feedback(&self) -> VgpDeviceForceFeedback {
        let ptr = self.raw_ptr as *const uinput_ff_upload;
        unsafe {
            VgpDeviceForceFeedback {
                id: (*ptr).effect.id,
                direction: (*ptr).effect.direction,
                replay: VgpDeviceForceFeedbackReplay {
                    length: (*ptr).effect.replay.length,
                    delay: (*ptr).effect.replay.delay,
                },
                r#type: if (*ptr).effect.type_ == FF_RUMBLE as u16 {
                    VgpDeviceForceFeedbackType::Rumble {
                        strong_magnitude: (*ptr).effect.u.rumble.strong_magnitude,
                        weak_magnitude: (*ptr).effect.u.rumble.weak_magnitude,
                    }
                } else {
                    VgpDeviceForceFeedbackType::Unsupported
                },
            }
        }
    }
}

impl Drop for ForceFeedbackUpload {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.raw_ptr, self.layout);
        }
    }
}

pub struct ForceFeedbackErase {
    layout: Layout,
    raw_ptr: *mut u8,
}

impl ForceFeedbackErase {
    pub fn new(request_id: u32) -> Self {
        let layout = Layout::new::<uinput_ff_erase>();
        let raw_ptr = unsafe { alloc(layout) };

        unsafe {
            (*(raw_ptr as *mut uinput_ff_erase)).request_id = request_id;
        }

        Self { layout, raw_ptr }
    }

    pub fn mut_ptr(&self) -> *mut uinput_ff_erase {
        self.raw_ptr as *mut uinput_ff_erase
    }

    pub fn set_retval(&mut self, retval: i32) {
        unsafe {
            (*(self.raw_ptr as *mut uinput_ff_erase)).retval = retval;
        }
    }

    pub fn get_effect_id(&self) -> u32 {
        unsafe { (*(self.raw_ptr as *const uinput_ff_erase)).effect_id }
    }
}

impl Drop for ForceFeedbackErase {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.raw_ptr, self.layout);
        }
    }
}
