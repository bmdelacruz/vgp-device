use crate::api::*;

use std::{
    os::raw::c_short,
    sync::{Arc, Mutex},
};

use vigem_client as vgm;
use vigem_client::ClientExt;

#[derive(Clone)]
pub struct Bus {
    client: Arc<Mutex<vgm::Client>>,
}

impl Bus {
    pub fn new() -> Result<Bus, Error> {
        let client = vgm::Client::new().map_with_vgp_error()?;
        let client = Arc::new(Mutex::new(client));

        Ok(Bus { client })
    }

    pub fn plug_in(&mut self) -> Result<Device, Error> {
        let device = self.client.plug_in().map_with_vgp_error()?;

        Ok(Device { device })
    }
}

pub struct Device {
    device: vgm::Device,
}

impl Device {
    pub fn put_input(&mut self, input: Input) -> Result<(), Error> {
        let input = match input {
            Input::Move { thumb_stick, x, y } => match thumb_stick {
                ThumbStick::Left => vgm::Input::MovedLeftThumbStick(
                    (32767f32 * x) as c_short,
                    (32767f32 * y) as c_short,
                ),
                ThumbStick::Right => vgm::Input::MovedRightThumbStick(
                    (32767f32 * x) as c_short,
                    (32767f32 * y) as c_short,
                ),
            },
            Input::Press(button) => match button {
                Button::DpadDown => vgm::Input::Pressed(vgm::Button::DpadDown),
                Button::DpadUp => vgm::Input::Pressed(vgm::Button::DpadUp),
                Button::DpadLeft => vgm::Input::Pressed(vgm::Button::DpadLeft),
                Button::DpadRight => vgm::Input::Pressed(vgm::Button::DpadRight),
                Button::North => vgm::Input::Pressed(vgm::Button::Y),
                Button::South => vgm::Input::Pressed(vgm::Button::A),
                Button::West => vgm::Input::Pressed(vgm::Button::B),
                Button::East => vgm::Input::Pressed(vgm::Button::X),
                Button::Start => vgm::Input::Pressed(vgm::Button::Start),
                Button::Select => vgm::Input::Pressed(vgm::Button::Guide),
                Button::TriggerLeft => vgm::Input::Pressed(vgm::Button::LeftShoulder),
                Button::TriggerRight => vgm::Input::Pressed(vgm::Button::RightShoulder),
                Button::TriggerLeft2 => vgm::Input::PressedLeftTrigger(127),
                Button::TriggerRight2 => vgm::Input::PressedRightTrigger(127),
                Button::ThumbStickLeft => vgm::Input::Pressed(vgm::Button::LeftThumb),
                Button::ThumbStickRight => vgm::Input::Pressed(vgm::Button::RightThumb),
            },
            Input::Release(button) => match button {
                Button::DpadDown => vgm::Input::Released(vgm::Button::DpadDown),
                Button::DpadUp => vgm::Input::Released(vgm::Button::DpadUp),
                Button::DpadLeft => vgm::Input::Released(vgm::Button::DpadLeft),
                Button::DpadRight => vgm::Input::Released(vgm::Button::DpadRight),
                Button::North => vgm::Input::Released(vgm::Button::Y),
                Button::South => vgm::Input::Released(vgm::Button::A),
                Button::West => vgm::Input::Released(vgm::Button::B),
                Button::East => vgm::Input::Released(vgm::Button::X),
                Button::Start => vgm::Input::Released(vgm::Button::Start),
                Button::Select => vgm::Input::Released(vgm::Button::Guide),
                Button::TriggerLeft => vgm::Input::Released(vgm::Button::LeftShoulder),
                Button::TriggerRight => vgm::Input::Released(vgm::Button::RightShoulder),
                Button::TriggerLeft2 => vgm::Input::PressedLeftTrigger(0),
                Button::TriggerRight2 => vgm::Input::PressedRightTrigger(0),
                Button::ThumbStickLeft => vgm::Input::Released(vgm::Button::LeftThumb),
                Button::ThumbStickRight => vgm::Input::Released(vgm::Button::RightThumb),
            },
        };
        self.device.put_input(input).map_with_vgp_error()
    }

    pub fn get_output(&mut self) -> Result<Output, Error> {
        match self.device.get_output() {
            Some(output) => match output {
                vgm::Output::Rumble(large_motor, small_motor) => Ok(Output::Rumble {
                    large_motor: large_motor.into(),
                    small_motor: small_motor.into(),
                }),
                vgm::Output::Led(_) => Ok(Output::Unsupported),
            },
            None => Ok(Output::None),
        }
    }

    pub fn unplug(self) -> Result<(), Error> {
        self.device.unplug().map_with_vgp_error()
    }
}
