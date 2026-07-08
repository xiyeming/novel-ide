-- migrations/001_initial.sql
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    genre TEXT,
    sub_genre TEXT,
    target_readers TEXT,
    total_chapters INTEGER DEFAULT 0,
    words_per_chapter INTEGER DEFAULT 3000,
    narrative_pov TEXT,
    story_structure TEXT,
    core_outline TEXT,
    world_settings TEXT,
    character_profiles TEXT,
    golden_finger TEXT,
    writing_constraints TEXT,
    style_constraints TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);

CREATE TABLE IF NOT EXISTS project_settings (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, key)
);

CREATE INDEX IF NOT EXISTS idx_settings_project ON project_settings(project_id);

CREATE TABLE IF NOT EXISTS global_settings (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
