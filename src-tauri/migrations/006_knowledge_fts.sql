-- migrations/006_knowledge_fts.sql
CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_documents_fts USING fts5(
    title,
    content,
    content=knowledge_documents,
    content_rowid=rowid
);

-- Triggers to keep FTS index in sync
CREATE TRIGGER IF NOT EXISTS knowledge_documents_ai AFTER INSERT ON knowledge_documents BEGIN
    INSERT INTO knowledge_documents_fts(rowid, title, content)
    VALUES (new.rowid, new.title, new.content);
END;

CREATE TRIGGER IF NOT EXISTS knowledge_documents_ad AFTER DELETE ON knowledge_documents BEGIN
    INSERT INTO knowledge_documents_fts(knowledge_documents_fts, rowid, title, content)
    VALUES('delete', old.rowid, old.title, old.content);
END;

CREATE TRIGGER IF NOT EXISTS knowledge_documents_au AFTER UPDATE ON knowledge_documents BEGIN
    INSERT INTO knowledge_documents_fts(knowledge_documents_fts, rowid, title, content)
    VALUES('delete', old.rowid, old.title, old.content);
    INSERT INTO knowledge_documents_fts(rowid, title, content)
    VALUES (new.rowid, new.title, new.content);
END;
