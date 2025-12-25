use super::data_loader;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct StageData {
    stage_id: Option<String>,      // Changed to Option<String>
    name: Option<String>,          // Changed to Option<String>
    deftime: Option<String>,       // Changed to Option<String>
    length: Option<String>,        // Changed to Option<String>
    surface_id: Option<String>,    // Changed to Option<String>
    short_country: Option<String>, // Changed to Option<String>
    author: Option<String>,        // Changed to Option<String>
    tarmac: Option<String>,        // Changed to Option<String>
    gravel: Option<String>,        // Changed to Option<String>
    snow: Option<String>,          // Changed to Option<String>
    new_update: Option<String>,    // Changed to Option<String>
    author_web: Option<String>,    // Changed to Option<String>
    author_note: Option<String>,   // Changed to Option<String>
    fattrib: Option<String>,       // Changed to Option<String>
}

#[tauri::command]
pub fn get_stages_data(app_handle: AppHandle) -> Result<String, String> {
    let stages_json = data_loader::read_data_file(&app_handle, "FILE_STAGES_DATA")?;

    let stages: Vec<StageData> = serde_json::from_str(&stages_json)
        .map_err(|e| format!("Failed to parse stages data JSON: {}", e))?;

    serde_json::to_string(&stages).map_err(|e| format!("Serialization failed: {}", e))
}
