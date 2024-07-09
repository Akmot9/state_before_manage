use std::sync::{Arc, Mutex};
use tauri::State;
use crate::state::SonarState;

#[tauri::command]
pub fn get_matrice(state: State<'_, Arc<Mutex<SonarState>>>) -> Result<String, String> {
    let locked_state = state.lock().map_err(|_| "Failed to lock state".to_string())?;
    match locked_state.get_matrice_data() {
        Ok(data) => {
            println!("Data: {}", data);
            Ok(data)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}

#[tauri::command(async)]
pub fn set_recording(state: State<'_, Arc<Mutex<SonarState>>>, recording: bool) -> Result<(), String> {
    let mut locked_state = state.lock().map_err(|_| "Failed to lock state".to_string())?;
    locked_state.set_recording(recording);
    Ok(())
}