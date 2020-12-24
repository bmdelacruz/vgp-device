mod bindings;
mod internal;
mod types;

use crate::*;

use bindings::consts::*;
use internal::*;
use types::*;

use nix::{errno::Errno, fcntl::OFlag, NixPath};

fn button_to_binding_const(button: VgpDeviceButton) -> u16 {
    match button {
        VgpDeviceButton::DpadDown => BTN_DPAD_DOWN as u16,
        VgpDeviceButton::DpadUp => BTN_DPAD_UP as u16,
        VgpDeviceButton::DpadLeft => BTN_DPAD_LEFT as u16,
        VgpDeviceButton::DpadRight => BTN_DPAD_RIGHT as u16,
        VgpDeviceButton::North => BTN_Y as u16,
        VgpDeviceButton::South => BTN_A as u16,
        VgpDeviceButton::West => BTN_X as u16,
        VgpDeviceButton::East => BTN_B as u16,
        VgpDeviceButton::Start => BTN_START as u16,
        VgpDeviceButton::Select => BTN_SELECT as u16,
        VgpDeviceButton::TriggerLeft => BTN_TL as u16,
        VgpDeviceButton::TriggerRight => BTN_TR as u16,
        VgpDeviceButton::TriggerLeft2 => BTN_TL2 as u16,
        VgpDeviceButton::TriggerRight2 => BTN_TR2 as u16,
        VgpDeviceButton::ThumbStickLeft => BTN_THUMBL as u16,
        VgpDeviceButton::ThumbStickRight => BTN_THUMBR as u16,
    }
}

pub struct VgpDeviceImpl {
    fd: i32,
}

impl VgpDevice for VgpDeviceImpl {
    fn new() -> Result<Self, VgpError> {
        let fd = "/dev/uinput"
            .with_nix_path(|p| {
                let flags = OFlag::O_RDWR | OFlag::O_NONBLOCK;
                let fd = unsafe { libc::open(p.as_ptr(), flags.bits()) };
                Errno::result(fd).to_vgp_result()
            })
            .to_vgp_result()??;

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
            ui_set_evbit(fd, EV_KEY as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_DPAD_UP as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_DPAD_DOWN as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_DPAD_LEFT as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_DPAD_RIGHT as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_X as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_Y as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_A as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_B as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_START as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_SELECT as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_THUMBL as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_THUMBR as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_TL as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_TL2 as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_TR as u64).to_vgp_result()?;
            ui_set_keybit(fd, BTN_TR2 as u64).to_vgp_result()?;

            ui_set_evbit(fd, EV_FF as u64).to_vgp_result()?;
            ui_set_ffbit(fd, FF_RUMBLE as u64).to_vgp_result()?;

            ui_set_evbit(fd, EV_ABS as u64).to_vgp_result()?;
            ui_set_absbit(fd, ABS_X as u64).to_vgp_result()?;
            ui_set_absbit(fd, ABS_RX as u64).to_vgp_result()?;
            ui_set_absbit(fd, ABS_Y as u64).to_vgp_result()?;
            ui_set_absbit(fd, ABS_RY as u64).to_vgp_result()?;

            ui_abs_setup(fd, abs_x_setup.const_ptr()).to_vgp_result()?;
            ui_abs_setup(fd, abs_y_setup.const_ptr()).to_vgp_result()?;
            ui_abs_setup(fd, abs_rx_setup.const_ptr()).to_vgp_result()?;
            ui_abs_setup(fd, abs_ry_setup.const_ptr()).to_vgp_result()?;

            ui_dev_setup(fd, setup.const_ptr()).to_vgp_result()?;
            ui_dev_create(fd).to_vgp_result()?;
        }

        Ok(Self { fd })
    }

    fn make_input(&mut self, input: VgpDeviceInput) -> Result<(), VgpError> {
        let time_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| {
                VgpError::Unknown(VgpUnknownError::new(format!(
                    "Cannot get system time! {}",
                    e
                )))
            })?;
        let time = (time_now.as_secs() as i64, time_now.subsec_micros() as i64);

        match input {
            VgpDeviceInput::PressButton(b) => {
                let press_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: button_to_binding_const(b),
                    value: 1,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd, press_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).to_vgp_result()?;
                }
            }
            VgpDeviceInput::ReleaseButton(b) => {
                let release_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: button_to_binding_const(b),
                    value: 0,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd, release_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).to_vgp_result()?;
                }
            }
            VgpDeviceInput::MoveThumbStick { thumb_stick, x, y } => {
                let x_change_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: match thumb_stick {
                        VgpDeviceThumbStick::Left => ABS_X as u16,
                        VgpDeviceThumbStick::Right => ABS_RX as u16,
                    },
                    value: (x * 512f32) as i32,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd, x_change_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).to_vgp_result()?;
                }
                let y_change_input_event = InputEvent::from(SafeInputEvent {
                    r#type: EV_KEY as u16,
                    code: match thumb_stick {
                        VgpDeviceThumbStick::Left => ABS_Y as u16,
                        VgpDeviceThumbStick::Right => ABS_RY as u16,
                    },
                    value: (y * 512f32) as i32,
                    time,
                });
                unsafe {
                    let n = libc::write(self.fd, y_change_input_event.c_ptr(), InputEvent::size());
                    Errno::result(n).to_vgp_result()?;
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
            let n = libc::write(self.fd, press_input_event.c_ptr(), InputEvent::size());
            Errno::result(n).to_vgp_result()?;
        }

        Ok(())
    }

    fn get_next_event(&mut self) -> Result<VgpDeviceEvent, VgpError> {
        let input_event = InputEvent::new();

        let n = unsafe {
            let n = libc::read(self.fd, input_event.c_ptr(), InputEvent::size());
            Errno::result(n).map(|n| n as usize)
        };

        match n {
            Ok(n) => {
                if n != InputEvent::size() {
                    Err(VgpError::Unknown(VgpUnknownError::new(format!(
                        "read error: expected read size {}, got {}",
                        InputEvent::size(),
                        n
                    ))))
                } else {
                    let input_event: SafeInputEvent = input_event.into();
                    if input_event.r#type == EV_UINPUT as u16 {
                        if input_event.code == UI_FF_UPLOAD as u16 {
                            let mut force_feedback_upload =
                                ForceFeedbackUpload::new(input_event.value as u32);

                            unsafe {
                                ui_begin_ff_upload(self.fd, force_feedback_upload.mut_ptr())
                                    .to_vgp_result()?;
                            }

                            force_feedback_upload.set_retval(0);

                            unsafe {
                                ui_end_ff_upload(self.fd, force_feedback_upload.mut_ptr())
                                    .to_vgp_result()?;
                            }

                            Ok(VgpDeviceEvent::ForceFeedbackUploaded(
                                force_feedback_upload.to_vgp_device_force_feedback(),
                            ))
                        } else if input_event.code == UI_FF_ERASE as u16 {
                            let mut force_feedback_erase =
                                ForceFeedbackErase::new(input_event.value as u32);

                            unsafe {
                                ui_begin_ff_erase(self.fd, force_feedback_erase.mut_ptr())
                                    .to_vgp_result()?;
                            }

                            force_feedback_erase.set_retval(0);

                            unsafe {
                                ui_end_ff_erase(self.fd, force_feedback_erase.mut_ptr())
                                    .to_vgp_result()?;
                            }

                            Ok(VgpDeviceEvent::ForceFeedbackErased(
                                force_feedback_erase.get_effect_id() as i16,
                            ))
                        } else {
                            log::warn!("Got an unsupported input event: {:?}", input_event);

                            Ok(VgpDeviceEvent::Unsupported)
                        }
                    } else if input_event.r#type == EV_FF as u16 {
                        if input_event.value == 0 {
                            Ok(VgpDeviceEvent::ForceFeedbackStopped(
                                input_event.code as i16,
                            ))
                        } else if input_event.value == 1 {
                            Ok(VgpDeviceEvent::ForceFeedbackPlayed(input_event.code as i16))
                        } else {
                            Err(VgpError::Unknown(VgpUnknownError::new(format!(
                                "Expected 0 or 1 for value of force feedback input event. Got {}. Input event: {:?}", input_event.value, input_event
                            ))))
                        }
                    } else {
                        log::warn!("Got an unsupported input event: {:?}", input_event);

                        Ok(VgpDeviceEvent::Unsupported)
                    }
                }
            }
            Err(e) => match e {
                nix::Error::Sys(Errno::EAGAIN) => Ok(VgpDeviceEvent::None),
                _ => Err(VgpError::Internal(e)),
            },
        }
    }
}

impl Drop for VgpDeviceImpl {
    fn drop(&mut self) {
        unsafe {
            ui_dev_destroy(self.fd).unwrap();
        }
        nix::unistd::close(self.fd).unwrap();
    }
}
