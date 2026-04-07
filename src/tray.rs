use hbb_common::{allow_err, log};

pub fn start_tray() {}

pub fn exit_tray() {}

#[cfg(windows)]
pub fn set_tray_white(_white: bool) {}

#[cfg(windows)]
pub fn refresh_tray() {}

#[cfg(target_os = "linux")]
pub fn set_app_icon(_icon: >k::Image) {}

#[cfg(windows)]
pub fn make_tray() -> hbb_common::ResultType<()> {
Ok(())
}

#[cfg(target_os = "macos")]
pub fn make_tray() -> hbb_common::ResultType<()> {
Ok(())
}

#[cfg(target_os = "linux")]
pub fn make_tray() -> hbb_common::ResultType<()> {
Ok(())
}

#[cfg(windows)]
pub fn show_main_window() {}

#[cfg(target_os = "macos")]
pub fn show_main_window() {}

#[cfg(target_os = "linux")]
pub fn show_main_window() {}
