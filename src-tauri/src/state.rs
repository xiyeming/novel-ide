// src-tauri/src/state.rs
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    db: Arc<RwLock<Option<SqlitePool>>>,
}

impl AppState {
    pub async fn set_db(&self, pool: SqlitePool) {
        *self.db.write().await = Some(pool);
    }

    pub async fn db(&self) -> Result<SqlitePool, crate::error::AppError> {
        self.db
            .read()
            .await
            .clone()
            .ok_or_else(|| crate::error::AppError::Internal("数据库未初始化".into()))
    }
}
