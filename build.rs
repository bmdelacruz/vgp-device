#[cfg(target_os = "linux")]
fn main() {
    let bindings = bindgen::builder()
        .header("bindings/linux_impl.h")
        .whitelist_var("EV_KEY")
        .whitelist_var("BTN_DPAD_UP")
        .whitelist_var("BTN_DPAD_DOWN")
        .whitelist_var("BTN_DPAD_RIGHT")
        .whitelist_var("BTN_DPAD_LEFT")
        .whitelist_var("BTN_X")
        .whitelist_var("BTN_Y")
        .whitelist_var("BTN_A")
        .whitelist_var("BTN_B")
        .whitelist_var("BTN_START")
        .whitelist_var("BTN_SELECT")
        .whitelist_var("BTN_THUMBL")
        .whitelist_var("BTN_THUMBR")
        .whitelist_var("BTN_TL")
        .whitelist_var("BTN_TL2")
        .whitelist_var("BTN_TR")
        .whitelist_var("BTN_TR2")
        .whitelist_var("EV_FF")
        .whitelist_var("FF_RUMBLE")
        .whitelist_var("FF_MAX_EFFECTS")
        .whitelist_var("EV_ABS")
        .whitelist_var("ABS_X")
        .whitelist_var("ABS_Y")
        .whitelist_var("ABS_RX")
        .whitelist_var("ABS_RY")
        .whitelist_var("EV_SYN")
        .whitelist_var("SYN_REPORT")
        .whitelist_var("EV_UINPUT")
        .whitelist_var("UI_FF_UPLOAD")
        .whitelist_var("UI_FF_ERASE")
        .whitelist_type("uinput_setup")
        .whitelist_type("uinput_abs_setup")
        .whitelist_type("input_event")
        .whitelist_type("uinput_ff_upload")
        .whitelist_type("uinput_ff_erase")
        .generate()
        .unwrap();

    let path = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&path);
    bindings.write_to_file(path.join("bindings.rs")).unwrap();
}

#[cfg(target_os = "windows")]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {}
