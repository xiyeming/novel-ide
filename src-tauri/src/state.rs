use sqlx::sqlite::SqlitePool;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: RwLock<Option<SqlitePool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db: RwLock::new(None),
        }
    }
}

impl AppState {
    pub async fn set_db(&self, pool: SqlitePool) {
        let mut db = self.db.write().await;
        *db = Some(pool);
    }

    pub async fn get_db(&self) -> Option<SqlitePool> {
        let db = self.db.read().await;
        db.clone()
    }
}
