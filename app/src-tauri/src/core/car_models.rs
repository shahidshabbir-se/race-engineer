use super::data_loader;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarModel {
    id: String,
    name: String,
    path: String,
    filename: String,
    hash: String,
    fallback: String,
}

#[tauri::command]
pub fn get_car_models(app_handle: AppHandle) -> Result<String, String> {
    let car_models_json = data_loader::read_data_file(&app_handle, "FILE_CAR_MODELS")?;

    let car_models: Vec<CarModel> = serde_json::from_str(&car_models_json)
        .map_err(|e| format!("Failed to parse car models JSON: {}", e))?;

    serde_json::to_string(&car_models).map_err(|_| "Failed to serialize car models".to_string())
}
