use super::data_loader;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarData {
    car_id: Option<String>,         // Changed to Option<String>
    power: Option<String>,          // Changed to Option<String>
    torque: Option<String>,         // Changed to Option<String>
    drive_train: Option<String>,    // Changed to Option<String>
    engine: Option<String>,         // Changed to Option<String>
    transmission: Option<String>,   // Changed to Option<String>
    weight: Option<String>,         // Changed to Option<String>
    wdf: Option<String>,            // Changed to Option<String>
    steering_wheel: Option<String>, // Changed to Option<String>
    skin: Option<String>,           // Changed to Option<String>
    model: Option<String>,          // Changed to Option<String>
    audio: Option<String>,          // Changed to Option<String>
    year: Option<String>,           // Changed to Option<String>
    shifter_type: Option<String>,   // Changed to Option<String>
    id: Option<String>,             // Changed to Option<String>
}

#[tauri::command]
pub fn get_car_data(app_handle: AppHandle) -> Result<String, String> {
    let raw = data_loader::read_data_file(&app_handle, "FILE_CARS_DATA")?;

    let car_data_list: Vec<CarData> =
        serde_json::from_str(&raw).map_err(|e| format!("Failed to parse car data JSON: {}", e))?;

    serde_json::to_string(&car_data_list)
        .map_err(|e| format!("Failed to serialize car data: {}", e))
}
