use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use super::data_loader;

#[derive(Serialize, Deserialize, Debug)]
struct CarGroupMapEntry {
    group_id: String,
    car_id: String,
    id: String,
    name: String,
    ngp: String,
}

#[tauri::command]
pub fn get_car_group_map(app_handle: AppHandle) -> Result<String, String> {
    let json_string = data_loader::read_data_file(&app_handle, "FILE_CAR_GROUP_MAP")?;

    let entries: Vec<CarGroupMapEntry> = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse car_group_map.json: {}", e))?;

    serde_json::to_string(&entries).map_err(|_| "Failed to serialize car group map".to_string())
}
