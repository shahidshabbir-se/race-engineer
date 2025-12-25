use super::data_loader;
use ini::Ini;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct CarOptions {
    car_id: String,
    name: String,
    rbrvr_seat0: String,
    rbrvr_seat1: String,
    rbrvr_seat2: String,
    fulldashposition_2d: String,
    fulldashposition_vr: String,
    fmod_mastervolume: String,
    setuptarmac: String,
    setupgravel: String,
    setupsnow: String,
    mirroredsteeringwheel: String,
    steeringrotation: String,
    forcefeedbacksensitivitytarmac: String,
    forcefeedbacksensitivitygravel: String,
    forcefeedbacksensitivitysnow: String,
}

#[tauri::command]
pub fn get_car_options(app_handle: AppHandle) -> Result<String, String> {
    let car_ini_path = data_loader::get_data_path(&app_handle, "FILE_RALLYSIMFANS_PERSONAL")?;

    let conf = Ini::load_from_file(&car_ini_path)
        .map_err(|e| format!("Failed to load .ini file: {}", e))?;

    let mut cars: Vec<CarOptions> = Vec::new();

    for (section_name, properties) in &conf {
        if section_name.is_none() && properties.is_empty() {
            continue;
        }

        let car_id = section_name
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let car = CarOptions {
            car_id,
            name: properties.get("name").unwrap_or("N/A").to_string(),
            rbrvr_seat0: properties.get("rbrvr_seat0").unwrap_or("N/A").to_string(),
            rbrvr_seat1: properties.get("rbrvr_seat1").unwrap_or("N/A").to_string(),
            rbrvr_seat2: properties.get("rbrvr_seat2").unwrap_or("N/A").to_string(),
            fulldashposition_2d: properties
                .get("fulldashposition_2d")
                .unwrap_or("N/A")
                .to_string(),
            fulldashposition_vr: properties
                .get("fulldashposition_vr")
                .unwrap_or("N/A")
                .to_string(),
            fmod_mastervolume: properties
                .get("fmod_mastervolume")
                .unwrap_or("N/A")
                .to_string(),
            setuptarmac: properties.get("setuptarmac").unwrap_or("N/A").to_string(),
            setupgravel: properties.get("setupgravel").unwrap_or("N/A").to_string(),
            setupsnow: properties.get("setupsnow").unwrap_or("N/A").to_string(),
            mirroredsteeringwheel: properties
                .get("mirroredsteeringwheel")
                .unwrap_or("N/A")
                .to_string(),
            steeringrotation: properties
                .get("steeringrotation")
                .unwrap_or("N/A")
                .to_string(),
            forcefeedbacksensitivitytarmac: properties
                .get("forcefeedbacksensitivitytarmac")
                .unwrap_or("N/A")
                .to_string(),
            forcefeedbacksensitivitygravel: properties
                .get("forcefeedbacksensitivitygravel")
                .unwrap_or("N/A")
                .to_string(),
            forcefeedbacksensitivitysnow: properties
                .get("forcefeedbacksensitivitysnow")
                .unwrap_or("N/A")
                .to_string(),
        };

        cars.push(car);
    }

    serde_json::to_string(&cars).map_err(|e| format!("Serialization failed: {}", e))
}
