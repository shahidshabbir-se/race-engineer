use super::data_loader;
use ini::Ini;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    key: String,
    value: String,
}

#[tauri::command]
pub fn get_settings(app_handle: AppHandle) -> Result<String, String> {
    let settings_ini_path = data_loader::get_data_path(&app_handle, "FILE_SETTINGS")?;

    let conf = Ini::load_from_file(settings_ini_path)
        .map_err(|_| "Failed to load the settings.ini file".to_string())?;

    if let Some(settings) = conf.section(Some("Settings")) {
        let settings_data: Vec<Settings> = settings
            .iter()
            .filter_map(|(key, value)| {
                if !key.starts_with('#') && !key.starts_with(';') {
                    Some(Settings {
                        key: key.to_string(),
                        value: value.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        serde_json::to_string(&settings_data)
            .map_err(|_| "Failed to serialize settings".to_string())
    } else {
        Err("No [Settings] section found in the settings.ini file".to_string())
    }
}
