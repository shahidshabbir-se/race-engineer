use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

/// Helper function to get the data directory path
/// Falls back to sample data if game directory is not configured or doesn't exist
pub fn get_data_path(app_handle: &AppHandle, file_env_key: &str) -> Result<PathBuf, String> {
    dotenv::dotenv().ok();

    let file_name = env::var(file_env_key)
        .map_err(|_| format!("Missing `{}` in .env file", file_env_key))?;

    // Try to get the configured RBR directory
    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    // Try reading from configured directory first
    if let Ok(settings_json) = fs::read_to_string(&store_path) {
        if let Ok(settings_dir) = serde_json::from_str::<serde_json::Value>(&settings_json) {
            if let Some(dir_path) = settings_dir.get("rbr_directory").and_then(|v| v.as_str()) {
                let full_path = Path::new(dir_path).join(&file_name);
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
        }
    }

    // Fallback to sample data
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // Go up from src-tauri to project root, then to data/rbr.samples
    let sample_path = current_dir
        .parent()
        .ok_or("Failed to get parent directory")?
        .parent()
        .ok_or("Failed to get project root")?
        .join("data")
        .join("rbr.samples")
        .join(&file_name);

    if sample_path.exists() {
        Ok(sample_path)
    } else {
        Err(format!(
            "File not found in game directory or sample data: {}",
            file_name
        ))
    }
}

/// Helper function to read file content with fallback to sample data
pub fn read_data_file(app_handle: &AppHandle, file_env_key: &str) -> Result<String, String> {
    let path = get_data_path(app_handle, file_env_key)?;
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))
}
