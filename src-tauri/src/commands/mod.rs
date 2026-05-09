use crate::db::{Database, SkillExecution, SkillPath, SkillStats};
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub db: Mutex<Database>,
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        let db = Database::new().map_err(|e| e.to_string())?;
        Ok(AppState { db: Mutex::new(db) })
    }
}

#[tauri::command]
pub fn get_skill_executions(
    state: State<AppState>,
    limit: Option<i64>,
) -> Result<Vec<SkillExecution>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_skill_executions(limit.unwrap_or(100))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skill_stats(
    state: State<AppState>,
    days: Option<i64>,
) -> Result<Vec<SkillStats>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_skill_stats(days.unwrap_or(7))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skill_paths(state: State<AppState>) -> Result<Vec<SkillPath>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_skill_paths().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_skill_path(
    state: State<AppState>,
    path: String,
    path_type: String,
    project_name: Option<String>,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.insert_skill_path(&path, &path_type, project_name.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_skill_path(state: State<AppState>, path: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_skill_path(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_skill_path(
    state: State<AppState>,
    path: String,
    enabled: bool,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_skill_path_enabled(&path, enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_skill_execution(
    state: State<AppState>,
    session_id: String,
    skill_id: String,
    args: Option<String>,
    trigger_type: Option<String>,
    status: Option<String>,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.insert_skill_execution(
        &session_id,
        &skill_id,
        args.as_deref(),
        trigger_type.as_deref().unwrap_or("ai_auto"),
        status.as_deref().unwrap_or("invoked"),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hook_script_path() -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    Ok(format!("{}/.claude/hooks/skill-hook.py", home))
}

#[tauri::command]
pub fn get_settings_path() -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    Ok(format!("{}/.claude/settings.json", home))
}

#[tauri::command]
pub fn get_skills_in_path(path: String) -> Result<Vec<String>, String> {
    let mut skills = Vec::new();
    let path = shellexpand::full(&path)
        .map_err(|e| e.to_string())?
        .into_owned();

    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() && !entry_path.file_name().unwrap().to_string_lossy().starts_with('.') {
                skills.push(entry_path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
    }
    Ok(skills)
}
