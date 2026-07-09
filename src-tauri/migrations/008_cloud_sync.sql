CREATE TABLE IF NOT EXISTS cloud_configs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL, -- webdav/oss/s3
    config TEXT NOT NULL, -- JSON encrypted config
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS cloud_sync_log (
    id TEXT PRIMARY KEY,
    config_id TEXT NOT NULL,
    action TEXT NOT NULL, -- upload/download/delete
    path TEXT NOT NULL,
    status TEXT NOT NULL, -- success/failed
    error TEXT,
    synced_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (config_id) REFERENCES cloud_configs(id) ON DELETE CASCADE
);
