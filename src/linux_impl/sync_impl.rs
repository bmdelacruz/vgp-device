use super::shared_impl::*;

pub struct Bus;

impl Bus {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    pub fn plug_in(&mut self) -> Result<Device, Error> {
        let (fd_reader, fd_writer) = create_uinput_device()?;
        Ok(Device {
            fd_reader,
            fd_writer,
        })
    }
}

pub struct Device {
    fd_reader: UInputFDReader,
    fd_writer: UInputFDWriter,
}

impl Device {
    pub fn put_input(&mut self, input: Input) -> Result<(), Error> {
        self.fd_writer.write(input)
    }

    pub fn get_output(&mut self) -> Result<Output, Error> {
        self.fd_reader.read()
    }

    pub fn unplug(self) -> Result<(), Error> {
        Ok(())
    }
}
