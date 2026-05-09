use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecution {
    pub id: i64,
    pub session_id: String,
    pub skill_id: String,
    pub args: Option<String>,
    pub trigger_type: String,
    pub context_summary: Option<String>,
    pub thinking_hint: Option<String>,
    pub status: String,
    pub timestamp: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPath {
    pub id: i64,
    pub path: String,
    pub path_type: String,
    pub project_name: Option<String>,
    pub enabled: bool,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub id: i64,
    pub session_id: String,
    pub user_prompt: String,
    pub skill_mentioned_in_thinking: Option<String>,
    pub skills_invoked: Option<String>,
    pub timestamp: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub id: i64,
    pub skill_id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub config: Option<String>,
    pub is_team_skill: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> DbResult<Self> {
        let db_path = Self::get_db_path()?;

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        let db = Database { conn };
        db.init_tables()?;
        db.init_default_paths()?; // 初始化默认路径
        Ok(db)
    }

    fn get_db_path() -> DbResult<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "devai", "skill-hub") {
            Ok(proj_dirs.data_dir().join("skills.db"))
        } else {
            Ok(PathBuf::from("skills.db"))
        }
    }

    fn init_tables(&self) -> DbResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS skill_executions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                skill_id TEXT NOT NULL,
                args TEXT,
                trigger_type TEXT NOT NULL DEFAULT 'ai_auto',
                context_summary TEXT,
                thinking_hint TEXT,
                status TEXT DEFAULT 'invoked',
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS skill_paths (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                path_type TEXT NOT NULL,
                project_name TEXT,
                enabled BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS session_context (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                user_prompt TEXT NOT NULL,
                skill_mentioned_in_thinking TEXT,
                skills_invoked TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS skill_definitions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                skill_id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                description TEXT,
                version TEXT DEFAULT '1.0.0',
                config TEXT,
                is_team_skill BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_skill_executions_skill_id ON skill_executions(skill_id)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_skill_executions_timestamp ON skill_executions(timestamp)",
            [],
        )?;

        Ok(())
    }

    pub fn insert_skill_execution(
        &self,
        session_id: &str,
        skill_id: &str,
        args: Option<&str>,
        trigger_type: &str,
        status: &str,
    ) -> DbResult<i64> {
        self.conn.execute(
            "INSERT INTO skill_executions (session_id, skill_id, args, trigger_type, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![session_id, skill_id, args, trigger_type, status],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_skill_executions(&self, limit: i64) -> DbResult<Vec<SkillExecution>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, skill_id, args, trigger_type, context_summary, thinking_hint, status, timestamp, created_at
             FROM skill_executions ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let executions = stmt.query_map([limit], |row| {
            Ok(SkillExecution {
                id: row.get(0)?,
                session_id: row.get(1)?,
                skill_id: row.get(2)?,
                args: row.get(3)?,
                trigger_type: row.get(4)?,
                context_summary: row.get(5)?,
                thinking_hint: row.get(6)?,
                status: row.get(7)?,
                timestamp: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(executions)
    }

    pub fn get_skill_stats(&self, days: i64) -> DbResult<Vec<SkillStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT skill_id, COUNT(*) as count, trigger_type
             FROM skill_executions
             WHERE timestamp >= datetime('now', ?1)
             GROUP BY skill_id, trigger_type
             ORDER BY count DESC"
        )?;

        let stats = stmt.query_map([format!("-{} days", days)], |row| {
            Ok(SkillStats {
                skill_id: row.get(0)?,
                count: row.get(1)?,
                trigger_type: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(stats)
    }

    pub fn insert_skill_path(&self, path: &str, path_type: &str, project_name: Option<&str>) -> DbResult<i64> {
        self.conn.execute(
            "INSERT INTO skill_paths (path, path_type, project_name) VALUES (?1, ?2, ?3) ON CONFLICT(path) DO UPDATE SET path_type=?2, project_name=?3",
            params![path, path_type, project_name],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_skill_paths(&self) -> DbResult<Vec<SkillPath>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, path_type, project_name, enabled, created_at FROM skill_paths ORDER BY created_at DESC"
        )?;

        let paths = stmt.query_map([], |row| {
            Ok(SkillPath {
                id: row.get(0)?,
                path: row.get(1)?,
                path_type: row.get(2)?,
                project_name: row.get(3)?,
                enabled: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(paths)
    }

    pub fn delete_skill_path(&self, path: &str) -> DbResult<()> {
        self.conn.execute("DELETE FROM skill_paths WHERE path = ?1", [path])?;
        Ok(())
    }

    pub fn update_skill_path_enabled(&self, path: &str, enabled: bool) -> DbResult<()> {
        self.conn.execute(
            "UPDATE skill_paths SET enabled = ?1 WHERE path = ?2",
            params![enabled, path],
        )?;
        Ok(())
    }

    pub fn init_default_paths(&self) -> DbResult<()> {
        let home = dirs::home_dir().unwrap_or_default();
        let user_path = home.join(".claude").join("skills");

        self.conn.execute(
            "INSERT OR IGNORE INTO skill_paths (path, path_type, enabled) VALUES (?1, 'user', TRUE)",
            params![user_path.to_string_lossy().to_string()],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStats {
    pub skill_id: String,
    pub count: i64,
    pub trigger_type: String,
}
