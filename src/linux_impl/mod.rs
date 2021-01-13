mod bindings;
mod internal;
mod types;

use std::collections::HashMap;

use crate::api::*;

use bindings::consts::*;
use internal::*;
use types::*;

use nix::{errno::Errno, fcntl::OFlag, NixPath};

fn button_to_binding_const(button: Button) -> u16 {
    match button {
        Button::DpadDown => BTN_DPAD_DOWN as u16,
        Button::DpadUp => BTN_DPAD_UP as u16,
        Button::DpadLeft => BTN_DPAD_LEFT as u16,
        Button::DpadRight => BTN_DPAD_RIGHT as u16,
        Button::North => BTN_Y as u16,
        Button::South => BTN_A as u16,
        Button::West => BTN_X as u16,
        Button::East => BTN_B as u16,
        Button::Start => BTN_START as u16,
        Button::Select => BTN_SELECT as u16,
        Button::TriggerLeft => BTN_TL as u16,
        Button::TriggerRight => BTN_TR as u16,
        Button::TriggerLeft2 => BTN_TL2 as u16,
        Button::TriggerRight2 => BTN_TR2 as u16,
        Button::ThumbStickLeft => BTN_THUMBL as u16,
        Button::ThumbStickRight => BTN_THUMBR as u16,
    }
}

struct UInputFD(i32);

impl UInputFD {
    fn new() -> Result<Self, nix::Error> {
        "/dev/uinput"
            .with_nix_path(|p| {
                let flags = OFlag::O_RDWR | OFlag::O_NONBLOCK;
                let fd = unsafe { libc::open(p.as_ptr(), flags.bits()) };
                Errno::result(fd)
            })?
            .map(|fd| UInputFD(fd))
    }
}

impl Drop for UInputFD {
    fn drop(&mut self) {
        if let Err(e) = nix::unistd::close(self.0) {
            log::error!("Failed to close device's file descriptor: {:?}", e);
        }
    }
}

pub struct Bus;

impl Bus {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    pub fn plug_in(&mut self) -> Result<Device, Error> {
        let fd = UInputFD::new().map_with_vgp_error()?;

        let abs_x_setup = AbsSetup::from(SafeAbsSetup {
            code: ABS_X as u16,
            value: 0,
            minimum: -512,
            maximum: 512,
            fuzz: 0,
            flat: 15,
            resolution: 0,
        });
        let abs_y_setup = AbsSetup::from(SafeAbsSetup {
            code: ABS_Y as u16,
            value: 0,
            minimum: -512,
            maximum: 512,
            fuzz: 0,
            flat: 15,
            resolution: 0,
        });
        let abs_rx_setup = AbsSetup::from(SafeAbsSetup {
            code: ABS_RX as u16,
            value: 0,
            minimum: -512,
            maximum: 512,
            fuzz: 0,
            flat: 15,
            resolution: 0,
        });
        let abs_ry_setup = AbsSetup::from(SafeAbsSetup {
            code: ABS_RY as u16,
            value: 0,
            minimum: -512,
            maximum: 512,
            fuzz: 0,
            flat: 15,
            resolution: 0,
        });
        let setup = Setup::from(SafeSetup {
            bustype: 0x06,
            version: 1,
            vendor: 0x0bdc,
            product: 0x4386,
            ff_effects_max: FF_MAX_EFFECTS as u32,
            name: "virtual gamepad (vgp)\0",
        });

        unsafe {
            ui_set_evbit(fd.0, EV_KEY as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_DPAD_UP as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_DPAD_DOWN as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_DPAD_LEFT as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_DPAD_RIGHT as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_X as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_Y as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_A as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_B as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_START as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_SELECT as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_THUMBL as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_THUMBR as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_TL as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_TL2 as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_TR as u64).map_with_vgp_error()?;
            ui_set_keybit(fd.0, BTN_TR2 as u64).map_with_vgp_error()?;

            ui_set_evbit(fd.0, EV_FF as u64).map_with_vgp_error()?;
            ui_set_ffbit(fd.0, FF_RUMBLE as u64).map_with_vgp_error()?;

            ui_set_evbit(fd.0, EV_ABS as u64).map_with_vgp_error()?;
            ui_set_absbit(fd.0, ABS_X as u64).map_with_vgp_error()?;
            ui_set_absbit(fd.0, ABS_RX as u64).map_with_vgp_error()?;
            ui_set_absbit(fd.0, ABS_Y as u64).map_with_vgp_error()?;
            ui_set_absbit(fd.0, ABS_RY as u64).map_with_vgp_error()?;

            ui_abs_setup(fd.0, abs_x_setup.const_ptr()).map_with_vgp_error()?;
            ui_abs_setup(fd.0, abs_y_setup.const_ptr()).map_with_vgp_error()?;
            ui_abs_setup(fd.0, abs_rx_setup.const_ptr()).map_with_vgp_error()?;
            ui_abs_setup(fd.0, abs_ry_setup.const_ptr()).map_with_vgp_error()?;

            ui_dev_setup(fd.0, setup.const_ptr()).map_with_vgp_error()?;
            ui_dev_create(fd.0).map_with_vgp_error()?;
        }

        Ok(Device {
            fd,
            ff_map: HashMap::new(),
        })
    }
}

pub struct Device {
    fd: UInputFD,
    ff_map: HashMap<u32, ForceFeedback>,
}

impl Device {
    pub fn put_input(&mut self, input: Input) -> Result<(), Error> {
        let time_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| Error::Unknown(format!("Cannot get system time! {}", e)))?;
        let time = (time_now.as_secs() as i64, time_now.subsec_micros() as i64);

        match input {
            Input::Press(button) => {
                let press_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: button_to_binding_const(button),
                    value: 1,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd.0, press_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).map_with_vgp_error()?;
                }
            }
            Input::Release(button) => {
                let release_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: button_to_binding_const(button),
                    value: 0,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd.0, release_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).map_with_vgp_error()?;
                }
            }
            Input::Move { thumb_stick, x, y } => {
                let x_change_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_ABS as u16,
                    code: match thumb_stick {
                        ThumbStick::Left => ABS_X as u16,
                        ThumbStick::Right => ABS_RX as u16,
                    },
                    value: (x * 512f32) as i32,
                    time,
                });
                unsafe {
                    let n =
                        libc::write(self.fd.0, x_change_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).map_with_vgp_error()?;
                }
                let y_change_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_ABS as u16,
                    code: match thumb_stick {
                        ThumbStick::Left => ABS_Y as u16,
                        ThumbStick::Right => ABS_RY as u16,
                    },
                    value: (y * 512f32) as i32,
                    time,
                });
                unsafe {
                    let n =
                        libc::write(self.fd.0, y_change_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).map_with_vgp_error()?;
                }
            }
        }

        let press_input_event = InputEvent::from(SafeInputEvent {
            r#type: EV_SYN as u16,
            code: SYN_REPORT as u16,
            value: 0,
            time,
        });
        unsafe {
            let n = libc::write(self.fd.0, press_input_event.c_ptr(), InputEvent::size());
            Errno::result(n).map_with_vgp_error()?;
        }

        Ok(())
    }

    pub fn get_output(&mut self) -> Result<Output, Error> {
        let input_event = InputEvent::new();

        let n = unsafe {
            let n = libc::read(self.fd.0, input_event.c_ptr(), InputEvent::size());
            Errno::result(n).map(|n| n as usize)
        };

        match n {
            Ok(n) => {
                if n != InputEvent::size() {
                    Err(Error::Unknown(format!(
                        "(get_output) Read error: Expected read size {}, got {}.",
                        InputEvent::size(),
                        n
                    )))
                } else {
                    let input_event: SafeInputEvent = input_event.into();
                    if input_event.r#type == EV_UINPUT as u16 {
                        if input_event.code == UI_FF_UPLOAD as u16 {
                            let mut force_feedback_upload =
                                ForceFeedbackUpload::new(input_event.value as u32);

                            unsafe {
                                ui_begin_ff_upload(self.fd.0, force_feedback_upload.mut_ptr())
                                    .map_with_vgp_error()?;
                            }

                            let (effect_id, force_feedback) = force_feedback_upload.get_data();

                            self.ff_map.insert(effect_id, force_feedback);

                            force_feedback_upload.set_retval(0);

                            unsafe {
                                ui_end_ff_upload(self.fd.0, force_feedback_upload.mut_ptr())
                                    .map_with_vgp_error()?;
                            }

                            Ok(Output::None)
                        } else if input_event.code == UI_FF_ERASE as u16 {
                            let mut force_feedback_erase =
                                ForceFeedbackErase::new(input_event.value as u32);

                            unsafe {
                                ui_begin_ff_erase(self.fd.0, force_feedback_erase.mut_ptr())
                                    .map_with_vgp_error()?;
                            }

                            self.ff_map.remove(&force_feedback_erase.get_effect_id());

                            force_feedback_erase.set_retval(0);

                            unsafe {
                                ui_end_ff_erase(self.fd.0, force_feedback_erase.mut_ptr())
                                    .map_with_vgp_error()?;
                            }

                            Ok(Output::None)
                        } else {
                            log::warn!("Got an unsupported input event: {:?}", input_event);

                            Ok(Output::Unsupported)
                        }
                    } else if input_event.r#type == EV_FF as u16 {
                        if input_event.value == 0 {
                            Ok(Output::Rumble {
                                large_motor: 0,
                                small_motor: 0,
                            })
                        } else if input_event.value == 1 {
                            self.ff_map.get(&(input_event.code as u32)).map_or_else(
                                || Ok(Output::None),
                                |ff| match ff {
                                    ForceFeedback::Rumble {
                                        large_motor,
                                        small_motor,
                                    } => Ok(Output::Rumble {
                                        large_motor: *large_motor,
                                        small_motor: *small_motor,
                                    }),
                                    ForceFeedback::Unsupported => Ok(Output::None),
                                },
                            )
                        } else {
                            Err(Error::Unknown(format!(
                                "Expected 0 or 1 for value of force feedback input event. Got {}. Input event: {:?}", input_event.value, input_event
                            )))
                        }
                    } else {
                        log::warn!("Got an unsupported input event: {:?}", input_event);

                        Ok(Output::Unsupported)
                    }
                }
            }
            Err(e) => match e {
                nix::Error::Sys(Errno::EAGAIN) => Ok(Output::None),
                _ => Err(Error::Internal(e)),
            },
        }
    }

    pub fn unplug(self) -> Result<(), Error> {
        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = ui_dev_destroy(self.fd.0) {
                log::error!("Failed to destroy device: {:?}", e);
            }
        }
    }
}
