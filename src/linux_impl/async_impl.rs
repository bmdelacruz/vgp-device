use std::{os::unix::io::RawFd, thread::JoinHandle};

use nix::{
    sys::epoll::{
        epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
    },
    unistd::close,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use super::shared_impl::*;

struct EPollFD(RawFd);

impl EPollFD {
    fn new() -> Result<EPollFD, Error> {
        let fd = epoll_create1(EpollCreateFlags::empty()).map_with_vgp_error()?;
        Ok(EPollFD(fd))
    }
}

impl Drop for EPollFD {
    fn drop(&mut self) {
        if let Err(e) = close(self.0) {
            log::warn!("Failed to close epoll fd: {:?}", e);
        }
    }
}

pub struct Bus;

impl Bus {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    pub fn plug_in(&mut self) -> Result<Device, Error> {
        let (mut fd_reader, fd_writer) = create_uinput_device()?;
        let (output_tx, output_rx) = unbounded_channel();

        let join_handle = std::thread::spawn(move || {
            let epoll_fd = EPollFD::new()?;
            let mut epoll_events = [EpollEvent::empty()];
            let mut epoll_event = EpollEvent::new(EpollFlags::EPOLLOUT | EpollFlags::EPOLLET, 0);

            epoll_ctl(
                epoll_fd.0,
                EpollOp::EpollCtlAdd,
                fd_reader.fd() as RawFd,
                Some(&mut epoll_event),
            )
            .map_with_vgp_error()?;

            loop {
                log::trace!("Polling output...");

                let event_count =
                    epoll_wait(epoll_fd.0, &mut epoll_events, 250).map_with_vgp_error()?;

                log::trace!("Polling stopped. Event count: {}", event_count);

                if output_tx.is_closed() {
                    log::trace!("The other half of the output sender is already closed.");
                    return Ok(());
                }
                if event_count == 0 {
                    continue;
                }

                match fd_reader.read() {
                    Ok(output) => match output {
                        Output::None => {
                            log::info!("Read completed but there was no output event to send.");
                        }
                        Output::Unsupported => {
                            log::info!("Received an unsupported output event; will not send it.");
                        }
                        output => match output_tx.send(output) {
                            Ok(_) => {
                                log::trace!("Successfully sent the output event.");
                            }
                            Err(e) => {
                                log::error!("Failed to send output from device. {:?}", e);
                                return Err(Error::Unknown(
                                    "Output tx is already closed.".to_string(),
                                ));
                            }
                        },
                    },
                    Err(e) => {
                        log::error!(
                            "An error occurred while reading output from device. {:?}",
                            e
                        );
                        return Err(e);
                    }
                }
            }
        });

        Ok(Device {
            fd_writer,
            output_rx,
            join_handle,
        })
    }
}

pub struct Device {
    fd_writer: UInputFDWriter,
    output_rx: UnboundedReceiver<Output>,
    join_handle: JoinHandle<Result<(), Error>>,
}

impl Device {
    pub fn put_input(&mut self, input: Input) -> Result<(), Error> {
        self.fd_writer.write(input)
    }

    pub async fn get_output(&mut self) -> Option<Output> {
        self.output_rx.recv().await
    }

    pub fn unplug(mut self) -> Result<(), Error> {
        self.output_rx.close();

        self.join_handle
            .join()
            .map_err(|e| Error::Unknown(format!("Child thread panicked! {:?}", e)))?
    }
}
