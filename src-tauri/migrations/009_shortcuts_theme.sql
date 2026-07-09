CREATE TABLE IF NOT EXISTS shortcuts (
    id TEXT PRIMARY KEY,
    action TEXT NOT NULL UNIQUE,
    key_binding TEXT NOT NULL,
    platform TEXT NOT NULL DEFAULT 'all', -- all/windows/macos/linux
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS themes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL, -- light/dark/custom
    config TEXT NOT NULL DEFAULT '{}', -- JSON CSS variables
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default shortcuts
INSERT OR IGNORE INTO shortcuts (id, action, key_binding, platform) VALUES
('default-save', '保存', 'CmdOrCtrl+S', 'all'),
('default-new-chapter', '新建章节', 'CmdOrCtrl+N', 'all'),
('default-search', '搜索', 'CmdOrCtrl+F', 'all'),
('default-export', '导出', 'CmdOrCtrl+E', 'all'),
('default-ai-chat', 'AI 聊天', 'CmdOrCtrl+Shift+A', 'all'),
('default-proofread', '校对', 'CmdOrCtrl+Shift+P', 'all'),
('default-toggle-sidebar', '切换侧边栏', 'CmdOrCtrl+B', 'all'),
('default-fullscreen', '全屏', 'F11', 'all');

-- Default themes
INSERT OR IGNORE INTO themes (id, name, type, config, is_active) VALUES
('dark', '深色', 'dark', '{}', 1),
('light', '亮色', 'light', '{}', 0);
