#!/usr/bin/env python3
"""
DevAI Skill Hub - Claude Code Hook Script
用于检测 Skill 调用并记录到本地数据库
"""

import json
import sqlite3
import sys
import re
import os
from pathlib import Path
from datetime import datetime

def get_db_path():
    """获取数据库路径"""
    home = Path.home()
    if sys.platform == "darwin":
        db_dir = home / "Library/Application Support/com.devai.skill-hub"
    elif sys.platform == "win32":
        db_dir = home / "AppData/Local/com.devai.skill-hub"
    else:
        db_dir = home / ".devai-skill-hub"

    db_dir.mkdir(parents=True, exist_ok=True)
    return db_dir / "skills.db"


def init_db(db_path):
    """初始化数据库表"""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS skill_executions (
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
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS skill_paths (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            path_type TEXT NOT NULL,
            project_name TEXT,
            enabled BOOLEAN DEFAULT TRUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    """)

    cursor.execute("""
        CREATE INDEX IF NOT EXISTS idx_skill_executions_skill_id ON skill_executions(skill_id)
    """)

    cursor.execute("""
        CREATE INDEX IF NOT EXISTS idx_skill_executions_timestamp ON skill_executions(timestamp)
    """)

    conn.commit()
    conn.close()


def extract_skill_id(tool_input: str) -> list:
    """从 Skill 工具输入中提取所有 Skill ID"""
    try:
        data = json.loads(tool_input)
        skill = data.get("skill", "")
        args = data.get("args", "")

        # 去除 / 前缀
        skill = skill.strip("/")

        if skill:
            return [(skill, args)]
        return []
    except json.JSONDecodeError:
        return []


def extract_skills_from_prompt(prompt: str) -> list:
    """从 prompt 中提取所有 /skill 格式的 Skill ID"""
    pattern = r'/\b([a-zA-Z0-9_-]+)\b'
    matches = re.findall(pattern, prompt)
    return list(set(matches))  # 去重


def get_trigger_type(session_id: str, skill_id: str) -> str:
    """判断触发类型：用户手动还是 AI 自动"""
    # 尝试从会话历史中查找
    # 如果用户在同一个 session 中明确输入了 /skill_id 则为 'user'
    # 否则默认为 'ai_auto'
    return 'ai_auto'


def record_skill_execution(db_path: str, session_id: str, skill_id: str, args: str = None, trigger_type: str = 'ai_auto'):
    """记录 Skill 执行到数据库"""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("""
        INSERT INTO skill_executions (session_id, skill_id, args, trigger_type, status)
        VALUES (?, ?, ?, ?, ?)
    """, (session_id, skill_id, args, trigger_type, 'invoked'))

    conn.commit()
    conn.close()


def process_hook_input():
    """处理 Hook 输入"""
    try:
        # 从 stdin 读取 Hook 输入
        input_data = json.loads(sys.stdin.read())
    except json.JSONDecodeError as e:
        print(f"Failed to parse hook input: {e}", file=sys.stderr)
        return
    except Exception as e:
        print(f"Error reading hook input: {e}", file=sys.stderr)
        return

    tool_name = input_data.get("tool_name", "")
    tool_input = input_data.get("tool_input", "")
    session_id = input_data.get("session_id", "")

    # 只处理 Skill 工具调用
    if tool_name != "Skill":
        return

    # 初始化数据库
    db_path = str(get_db_path())
    init_db(db_path)

    # 提取 Skill ID
    skills = extract_skill_id(tool_input)

    for skill_id, args in skills:
        if skill_id:
            trigger_type = get_trigger_type(session_id, skill_id)
            try:
                record_skill_execution(db_path, session_id, skill_id, args, trigger_type)
                print(f"Recorded skill execution: {skill_id}", file=sys.stderr)
            except Exception as e:
                print(f"Failed to record skill execution: {e}", file=sys.stderr)


if __name__ == "__main__":
    process_hook_input()
