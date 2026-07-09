-- 007_workflow_agent.sql
CREATE TABLE IF NOT EXISTS workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    stages TEXT NOT NULL DEFAULT '[]', -- JSON array of stage configs
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS workflow_executions (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    chapter_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending/running/completed/failed
    current_stage INTEGER NOT NULL DEFAULT 0,
    results TEXT NOT NULL DEFAULT '{}', -- JSON map of stage results
    error TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    role TEXT NOT NULL, -- outline/writer/proofreader/editor/custom
    system_prompt TEXT NOT NULL,
    model_provider_id TEXT,
    temperature REAL NOT NULL DEFAULT 0.7,
    max_tokens INTEGER NOT NULL DEFAULT 2000,
    knowledge_base_ids TEXT NOT NULL DEFAULT '[]', -- JSON array of KB IDs
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_workflow_executions_workflow_id ON workflow_executions(workflow_id);
CREATE INDEX IF NOT EXISTS idx_workflow_executions_chapter_id ON workflow_executions(chapter_id);
