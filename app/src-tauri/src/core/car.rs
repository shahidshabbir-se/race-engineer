use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use super::data_loader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    id: String,
    name: String,
    path: String,
    hash: String,
    carmodel_id: String,
    user_id: String,
    base_group_id: String,
    test: String,
    ngp: String,
    custom_setups: String,
    rev: String,
    audio: Option<String>,
    audio_hash: String,
}

#[tauri::command]
pub fn get_car(app_handle: AppHandle) -> Result<String, String> {
    let car_json = data_loader::read_data_file(&app_handle, "FILE_CARS")?;

    let cars: Vec<Car> = serde_json::from_str(&car_json)
        .map_err(|e| format!("Failed to parse car JSON file: {}", e))?;

    serde_json::to_string(&cars).map_err(|e| format!("Failed to serialize car data: {}", e))
}
