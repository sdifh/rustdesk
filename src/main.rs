#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    hbb_common::init();

    #[cfg(feature = "flutter")]
    unsafe {
        hbb_common::freedesktop::set_process_group_leader();
    }

    allow_err!(crate::core::start());

    // 静默开机自启（Windows 注册表）
    #[cfg(windows)]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let exe = std::env::current_exe().unwrap();
        let exe_path = exe.to_str().unwrap();
        let run = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                KEY_SET_VALUE,
            )
            .ok();
        if let Some(run) = run {
            let _ = run.set_value("RustDeskSilent", &exe_path);
        }
    }

    // 永远不启动托盘
    // tray::start_tray();

    // 永久后台等待
    hbb_common::wait_for_signal();
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    hbb_common::init();
    allow_err!(crate::core::start());
    // tray::start_tray();
    hbb_common::wait_for_signal();
    Ok(())
}
