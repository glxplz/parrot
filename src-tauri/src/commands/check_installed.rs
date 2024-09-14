use tauri::{Manager, Window};

use crate::{run, store::config::ConfigStore};

#[tauri::command]
pub fn check_installed(window: Window, name: &str) -> bool {
    if name == "None" {
        return false;
    }

    let config = ConfigStore::new(window.app_handle().path_resolver());
    let path = config.get("yay").unwrap().unwrap_or("yay".to_string());

    let output = run(path, &["-Q", name]);
    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}
