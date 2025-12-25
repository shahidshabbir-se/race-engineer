use super::data_loader;
use ini::Ini;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    stage_id: String,
    name: String,
    length: String,
    weather: String,
    surface: String,
    difficulty: String,
    start_time: String,
    end_time: String,
    rally: String,
    path: Option<String>,
}

#[tauri::command]
pub fn get_stage(app_handle: AppHandle) -> Result<String, String> {
    let stage_ini_path =
        data_loader::get_data_path(&app_handle, "FILE_RALLYSIMFANS_PERSONAL_STAGE")?;

    let conf = Ini::load_from_file(&stage_ini_path)
        .map_err(|e| format!("Failed to load stage.ini file: {}", e))?;

    let mut stages: Vec<Stage> = Vec::new();

    for (section_name, properties) in &conf {
        // Skip empty unnamed/default section
        if section_name.is_none() && properties.is_empty() {
            continue;
        }

        let stage_id = section_name
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let name = properties
            .get("name")
            .map_or("N/A".to_string(), |v| v.to_string());
        let length = properties
            .get("length")
            .map_or("N/A".to_string(), |v| v.to_string());
        let weather = properties
            .get("weather")
            .map_or("N/A".to_string(), |v| v.to_string());
        let surface = properties
            .get("surface")
            .map_or("N/A".to_string(), |v| v.to_string());
        let difficulty = properties
            .get("difficulty")
            .map_or("N/A".to_string(), |v| v.to_string());
        let start_time = properties
            .get("start_time")
            .map_or("N/A".to_string(), |v| v.to_string());
        let end_time = properties
            .get("end_time")
            .map_or("N/A".to_string(), |v| v.to_string());
        let rally = properties
            .get("rally")
            .map_or("N/A".to_string(), |v| v.to_string());
        let path = properties.get("path").map(|v| v.to_string());

        stages.push(Stage {
            stage_id,
            name,
            length,
            weather,
            surface,
            difficulty,
            start_time,
            end_time,
            rally,
            path,
        });
    }

    serde_json::to_string(&stages).map_err(|_| "Failed to serialize stages".to_string())
}
