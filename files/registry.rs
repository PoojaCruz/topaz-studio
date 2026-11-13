//! Windows registry read helpers for Topaz Studio.

pub const APP_KEY: &str = "Software\\Topaz\\Topaz Studio";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Topaz Studio";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Topaz Studio") },
        RegValue { name: "Version",     value: String::from("v4.4.6") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Topaz Studio") },
        RegValue { name: "Publisher",       value: String::from("Topaz") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Topaz Studio\\uninstall.exe\"") },
    ]
}
