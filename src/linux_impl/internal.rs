const UI_IOC_MAGIC: u8 = b'U';

nix::ioctl_write_int!(ui_set_evbit, UI_IOC_MAGIC, 100);

nix::ioctl_write_int!(ui_set_keybit, UI_IOC_MAGIC, 101);

nix::ioctl_write_int!(ui_set_absbit, UI_IOC_MAGIC, 103);

nix::ioctl_write_int!(ui_set_ffbit, UI_IOC_MAGIC, 107);

nix::ioctl_none!(ui_dev_create, UI_IOC_MAGIC, 1);

nix::ioctl_none!(ui_dev_destroy, UI_IOC_MAGIC, 2);

nix::ioctl_write_ptr!(ui_dev_setup, UI_IOC_MAGIC, 3, super::bindings::uinput_setup);

nix::ioctl_write_ptr!(
    ui_abs_setup,
    UI_IOC_MAGIC,
    4,
    super::bindings::uinput_abs_setup
);

nix::ioctl_readwrite!(
    ui_begin_ff_upload,
    UI_IOC_MAGIC,
    200,
    super::bindings::uinput_ff_upload
);

nix::ioctl_write_ptr!(
    ui_end_ff_upload,
    UI_IOC_MAGIC,
    201,
    super::bindings::uinput_ff_upload
);

nix::ioctl_readwrite!(
    ui_begin_ff_erase,
    UI_IOC_MAGIC,
    202,
    super::bindings::uinput_ff_erase
);

nix::ioctl_write_ptr!(
    ui_end_ff_erase,
    UI_IOC_MAGIC,
    203,
    super::bindings::uinput_ff_erase
);
