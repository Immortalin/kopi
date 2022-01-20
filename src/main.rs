use std::fmt::Display;
use std::process::Command;
use std::ptr;
use windows::core::GUID;
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerReadACValueIndex, PowerReadDescription, PowerWriteACValueIndex,
};
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::SystemServices::GUID_SLEEP_SUBGROUP;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
// use windows::UI::Xaml::Controls::ContentDialog;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    let mut power_scheme: *mut windows::core::GUID = ptr::null_mut();
    let power_scheme_ptr: *mut *mut windows::core::GUID = &mut power_scheme;
    let mut sleep_timeout: u32 = 0;
    let sleep_timeout_ptr: *mut u32 = &mut sleep_timeout;
    let new_sleep_timeout: u32;
    // STANDBYIDLE GUID
    const STANDBYIDLE: &str = "29f6c1db-86da-48c5-9fdb-f2b67b1f44da";
    // let mut current_power_scheme: String;

    // let mut power_description: *mut u8 = ptr::null_mut();
    unsafe {
        PowerGetActiveScheme(HKEY::default(), power_scheme_ptr);
        // current_power_scheme = format!("{:x?}", *power_scheme).to_ascii_lowercase();
        PowerReadACValueIndex(
            HKEY::default(),
            power_scheme,
            &GUID_SLEEP_SUBGROUP,
            &GUID::from(STANDBYIDLE),
            sleep_timeout_ptr,
        );
        if sleep_timeout == 0 {
            new_sleep_timeout = 600;
            PowerWriteACValueIndex(
                HKEY::default(),
                power_scheme,
                &GUID_SLEEP_SUBGROUP,
                &GUID::from(STANDBYIDLE),
                new_sleep_timeout,
            );
        } else {
            new_sleep_timeout = 0;
            PowerWriteACValueIndex(
                HKEY::default(),
                power_scheme,
                &GUID_SLEEP_SUBGROUP,
                &GUID::from(STANDBYIDLE),
                new_sleep_timeout,
            );
        }
    }
    println!("{}", sleep_timeout);
    // let my_msgbox = ContentDialog::new().unwrap();
    // my_msgbox.SetTitle(IInspectable::try_from("Kopi").unwrap());
    // my_msgbox.ShowAsync();
    // unsafe {
    //     println!("{:?}", *power_scheme);
    // }
    let sleep_timeout_active = if sleep_timeout == 0 { true } else { false };
    let curr_sleep_timeout_min = sleep_timeout / 60;
    let new_sleep_timeout_min = new_sleep_timeout / 60;

    match Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Current Power Scheme")
        .text1(
            &format!("Sleep timeout is active: {sleep_timeout_active}\nCurrent sleep timeout: {curr_sleep_timeout_min} min\nNew sleep timeout: {new_sleep_timeout_min} min"), // &current_power_scheme,
        )
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
    {
        Err(e) => toast_notification_failure(e),

        Ok(_) => (),
    }
}

fn toast_notification_failure<T: Display>(e: T) {
    unsafe {
        MessageBoxA(
            None,
            format!("Toast notification failure! Error: {e}\nPlease contact support."),
            "Kopi",
            MB_OK,
        );
    }
}

// fn toggle_sleep_timeout(active: bool) -> std::result::Result<std::process::Output, std::io::Error> {
//     if active {
//         let output = Command::new("powercfg")
//             .args(["/change", "standby-timeout-ac", "10"])
//             .output();
//         output
//     } else {
//         let output = Command::new("powercfg")
//             .args(["/change", "standby-timeout-ac", "10"])
//             .output();
//         output
//     }
// }

#[cfg(test)]
mod tests {
    use super::toast_notification_failure;
    #[test]
    fn notification_failure_msgbox() {
        toast_notification_failure("testing error msgbox");
    }
}
