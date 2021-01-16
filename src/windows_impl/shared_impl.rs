use std::{
    collections::VecDeque,
    ffi::c_void,
    marker::PhantomData,
    os::raw::{c_short, c_uchar, c_ushort},
    sync::Arc,
};

use crate::api::{Button, Input, Output};

use super::errors::*;

pub use super::bindings::*;

pub struct RawOutput {
    pub large_motor: c_uchar,
    pub small_motor: c_uchar,
    pub led_number: c_uchar,
}

pub struct OutputQueue {
    latest_raw_output: RawOutput,
    queue: VecDeque<Output>,
}

impl OutputQueue {
    pub fn new() -> Self {
        Self {
            latest_raw_output: RawOutput {
                large_motor: 0,
                small_motor: 0,
                led_number: 0,
            },
            queue: VecDeque::new(),
        }
    }

    pub fn get(&mut self) -> Option<Output> {
        self.queue.pop_front()
    }

    pub fn put_and_get(&mut self, raw_output: RawOutput) -> Option<Output> {
        let did_large_motor_change = raw_output.large_motor != self.latest_raw_output.large_motor;
        let did_small_motor_change = raw_output.small_motor != self.latest_raw_output.small_motor;
        let did_led_number_change = raw_output.led_number != self.latest_raw_output.led_number;
        let has_queued_output = self.queue.front().is_some();

        self.latest_raw_output = raw_output;

        let output = if did_large_motor_change || did_small_motor_change {
            let rumble_output = Output::Rumble {
                large_motor: self.latest_raw_output.large_motor as u16,
                small_motor: self.latest_raw_output.small_motor as u16,
            };
            if has_queued_output {
                // queue the rumble output for later
                self.queue.push_back(rumble_output);
                // get the latest queued output
                self.queue.pop_front()
            } else {
                // we don't need to queue the rumble output since the queue is empty anyway
                Some(rumble_output)
            }
        } else if has_queued_output {
            // there is no rumble output and the queue is not empty
            self.queue.pop_front()
        } else {
            // there is no rumble output and the queue is empty
            Some(Output::None)
        };

        if did_led_number_change {
            let led_output = Output::Unsupported;
            if output.is_some() {
                // queue the led output for later
                self.queue.push_back(led_output);
                // return the rumble output or the latest queued output here
                output
            } else {
                // we don't need to queue the rumble output since the queue is empty
                // and there is no rumble output
                Some(led_output)
            }
        } else {
            // there is no led output so just return the rumble output
            // or the latest queued output here
            output
        }
    }
}

pub struct Client(PVIGEM_CLIENT);

impl Client {
    pub fn new() -> Self {
        let client_ptr = unsafe { vigem_alloc() };
        Self(client_ptr)
    }

    pub fn connect(self) -> Result<ConnectedClient, Error> {
        let err = unsafe { vigem_connect(self.0) };
        err.to_error().map_or_else(
            || {
                Ok(ConnectedClient {
                    client: Arc::new(self),
                })
            },
            |e| Err(e),
        )
    }
}

unsafe impl Send for Client {}

unsafe impl Sync for Client {}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe { vigem_free(self.0) }
    }
}

pub struct ConnectedClient {
    client: Arc<Client>,
}

impl ConnectedClient {
    pub fn add_target<T>(
        &mut self,
        notification_fn: PFN_VIGEM_X360_NOTIFICATION,
        notification_fn_data: T,
    ) -> Result<AddedTarget<T>, (Option<T>, Error)> {
        let target = Target::new();

        let err = unsafe { vigem_target_add(self.client.0, target.0) };
        err.to_error().map_or_else(|| Ok(()), |e| Err((None, e)))?;

        let data_ptr = Box::into_raw(Box::new(notification_fn_data));

        let err = unsafe {
            vigem_target_x360_register_notification(
                self.client.0,
                target.0,
                notification_fn,
                data_ptr as *mut c_void,
            )
        };
        err.to_error().map_or_else(
            || Ok(()),
            |e| {
                let data = unsafe { *Box::from_raw(data_ptr) };
                Err((Some(data), e))
            },
        )?;

        Ok(AddedTarget {
            target,
            report: XUSB_REPORT {
                wButtons: 0,
                bLeftTrigger: 0,
                bRightTrigger: 0,
                sThumbLX: 0,
                sThumbLY: 0,
                sThumbRX: 0,
                sThumbRY: 0,
            },
            client: Arc::clone(&self.client),
            _data: PhantomData::<T>::default(),
        })
    }
}

impl Drop for ConnectedClient {
    fn drop(&mut self) {
        unsafe { vigem_disconnect(self.client.0) }
    }
}

struct Target(PVIGEM_TARGET);

impl Target {
    fn new() -> Self {
        let target = unsafe { vigem_target_x360_alloc() };
        Self(target)
    }
}

unsafe impl Send for Target {}

unsafe impl Sync for Target {}

impl Drop for Target {
    fn drop(&mut self) {
        unsafe { vigem_target_free(self.0) }
    }
}

pub struct AddedTarget<T> {
    client: Arc<Client>,
    target: Target,
    report: XUSB_REPORT,
    _data: PhantomData<T>,
}

impl<T> AddedTarget<T> {
    pub fn send_report(&mut self, input: Input) -> Result<(), Error> {
        match input {
            Input::Press(Button::TriggerLeft2) => self.report.bLeftTrigger = 127,
            Input::Press(Button::TriggerRight2) => self.report.bRightTrigger = 127,
            Input::Press(button) => {
                let raw_button = match button {
                    Button::DpadDown => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_DOWN,
                    Button::DpadUp => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_UP,
                    Button::DpadLeft => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_LEFT,
                    Button::DpadRight => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_RIGHT,
                    Button::North => _XUSB_BUTTON_XUSB_GAMEPAD_Y,
                    Button::South => _XUSB_BUTTON_XUSB_GAMEPAD_A,
                    Button::West => _XUSB_BUTTON_XUSB_GAMEPAD_X,
                    Button::East => _XUSB_BUTTON_XUSB_GAMEPAD_B,
                    Button::Start => _XUSB_BUTTON_XUSB_GAMEPAD_START,
                    Button::Select => _XUSB_BUTTON_XUSB_GAMEPAD_GUIDE,
                    Button::TriggerLeft => _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_SHOULDER,
                    Button::TriggerRight => _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_SHOULDER,
                    Button::ThumbStickLeft => _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_THUMB,
                    Button::ThumbStickRight => _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_THUMB,
                    _ => 1,
                };
                self.report.wButtons |= raw_button as c_ushort;
            }
            Input::Release(Button::TriggerLeft2) => self.report.bLeftTrigger = 0,
            Input::Release(Button::TriggerRight2) => self.report.bRightTrigger = 0,
            Input::Release(button) => {
                let raw_button = match button {
                    Button::DpadDown => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_DOWN,
                    Button::DpadUp => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_UP,
                    Button::DpadLeft => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_LEFT,
                    Button::DpadRight => _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_RIGHT,
                    Button::North => _XUSB_BUTTON_XUSB_GAMEPAD_Y,
                    Button::South => _XUSB_BUTTON_XUSB_GAMEPAD_A,
                    Button::West => _XUSB_BUTTON_XUSB_GAMEPAD_X,
                    Button::East => _XUSB_BUTTON_XUSB_GAMEPAD_B,
                    Button::Start => _XUSB_BUTTON_XUSB_GAMEPAD_START,
                    Button::Select => _XUSB_BUTTON_XUSB_GAMEPAD_GUIDE,
                    Button::TriggerLeft => _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_SHOULDER,
                    Button::TriggerRight => _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_SHOULDER,
                    Button::ThumbStickLeft => _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_THUMB,
                    Button::ThumbStickRight => _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_THUMB,
                    _ => 0,
                };
                self.report.wButtons &= !raw_button as c_ushort;
            }
            Input::Move { thumb_stick, x, y } => match thumb_stick {
                crate::ThumbStick::Left => {
                    self.report.sThumbLX = (32767f32 * x) as c_short;
                    self.report.sThumbLY = (32767f32 * y) as c_short;
                }
                crate::ThumbStick::Right => {
                    self.report.sThumbRX = (32767f32 * x) as c_short;
                    self.report.sThumbRY = (32767f32 * y) as c_short;
                }
            },
        }

        let err = unsafe { vigem_target_x360_update(self.client.0, self.target.0, self.report) };
        err.to_error().map_or_else(|| Ok(()), |e| Err(e))
    }

    pub fn remove(self) -> (T, Result<(), Error>) {
        let data = {
            let _guard = DeviceNotificationLockGuard::new(&self.target.0);
            unsafe {
                let data_ptr = vigem_target_x360_unregister_notification(self.target.0);
                *Box::from_raw(data_ptr as *mut T)
            }
        };

        let err = unsafe { vigem_target_remove(self.client.0, self.target.0) };
        let res = err.to_error().map_or_else(|| Ok(()), |e| Err(e));

        (data, res)
    }
}

/// Since we aren't really going to use the notification data here,
/// it's safe to say that AddedTarget is Sync even if T is not.
unsafe impl<T> Sync for AddedTarget<T> {}

struct DeviceNotificationLockGuard<'a> {
    target: &'a PVIGEM_TARGET,
}

impl<'a> DeviceNotificationLockGuard<'a> {
    pub fn new(target: &'a PVIGEM_TARGET) -> Self {
        unsafe {
            vigem_target_lock_notification(*target);
        }
        Self { target }
    }
}

impl<'a> Drop for DeviceNotificationLockGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            vigem_target_unlock_notification(*self.target);
        }
    }
}
