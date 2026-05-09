mod commands;
mod db;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app_state = AppState::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_skill_executions,
            commands::get_skill_stats,
            commands::get_skill_paths,
            commands::add_skill_path,
            commands::remove_skill_path,
            commands::toggle_skill_path,
            commands::record_skill_execution,
            commands::get_hook_script_path,
            commands::get_settings_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
