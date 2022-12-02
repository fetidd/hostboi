#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_favorites() -> Result<Vec<String>, String> {
    Ok(vec![String::from("DEV51")])
}

#[tauri::command]
fn swap(box_number: i32) -> Result<(), String> {
    
    Ok(())
}

#[tauri::command]
fn favorite(favorite: String) -> Result<(), String> {

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_favorites, swap, favorite])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
