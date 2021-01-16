use std::{ffi::c_void, os::raw::c_uchar};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::api::*;

use super::shared_impl::*;

unsafe extern "C" fn notification_callback(
    _client: PVIGEM_CLIENT,
    _target: PVIGEM_TARGET,
    large_motor: c_uchar,
    small_motor: c_uchar,
    led_number: c_uchar,
    user_data: *mut c_void,
) {
    // NOTE: do not touch the client and target here!!!

    let raw_output_tx_ptr = user_data as *mut UnboundedSender<RawOutput>;

    if let Err(e) = (*raw_output_tx_ptr).send(RawOutput {
        large_motor,
        small_motor,
        led_number,
    }) {
        log::error!(
            "Got an error sending the raw output from the ViGEm callback. Error: {}",
            e
        );
    }
}

pub struct Bus {
    client: ConnectedClient,
}

impl Bus {
    pub fn new() -> Result<Bus, Error> {
        let client = Client::new();
        let client = client.connect().map_with_vgp_error()?;

        Ok(Bus { client })
    }

    pub fn plug_in(&mut self) -> Result<Device, Error> {
        let (raw_output_tx, raw_output_rx) = unbounded_channel::<RawOutput>();

        let result = self
            .client
            .add_target(Some(notification_callback), raw_output_tx);

        let target = match result {
            Ok(target) => target,
            Err((_, e)) => return Err(e).map_with_vgp_error(),
        };

        Ok(Device {
            target,
            raw_output_rx,
            output_queue: OutputQueue::new(),
        })
    }
}

pub struct Device {
    target: AddedTarget<UnboundedSender<RawOutput>>,
    raw_output_rx: UnboundedReceiver<RawOutput>,
    output_queue: OutputQueue,
}

impl Device {
    pub fn put_input(&mut self, input: Input) -> Result<(), Error> {
        self.target.send_report(input).map_with_vgp_error()
    }

    pub async fn get_output(&mut self) -> Option<Output> {
        match self.raw_output_rx.recv().await {
            Some(output) => self.output_queue.put_and_get(output),
            None => self.output_queue.get(),
        }
    }

    pub fn unplug(self) -> Result<(), Error> {
        let (_, result) = self.target.remove();
        result.map_with_vgp_error()
    }
}
