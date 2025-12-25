use super::data_loader;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
struct CarGroup {
    id: String,
    name: String,
    user_id: String,
    main: String,
    test: String,
    ngp: String,
}

#[tauri::command]
pub fn get_car_groups(app_handle: AppHandle) -> Result<String, String> {
    let json_data = data_loader::read_data_file(&app_handle, "FILE_CAR_GROUPS")?;

    let car_groups: Vec<CarGroup> =
        serde_json::from_str(&json_data).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    serde_json::to_string(&car_groups).map_err(|_| "Failed to serialize car groups".to_string())
}
