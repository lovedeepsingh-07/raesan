use sqlx::sqlite;
use std::str::FromStr;
use tokio::sync::{RwLock, watch};

#[derive(Debug)]
pub struct AppState {
    pub db_state: DBState,
    pub db_pool: sqlite::SqlitePool,
}
impl AppState {
    pub async fn new(db_path: &str) -> Result<Self, error::Error> {
        let db_options = sqlite::SqliteConnectOptions::from_str(db_path)?.create_if_missing(false);
        let db_pool = sqlite::SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;
        Ok(AppState {
            db_state: DBState {
                populate_cancel_tx: None,
            },
            db_pool,
        })
    }
}
pub struct AppStateGuard<'a>(pub &'a RwLock<AppState>);
impl Drop for AppStateGuard<'_> {
    fn drop(&mut self) {
        if let Ok(mut state) = self.0.try_write() {
            state.db_state.populate_cancel_tx = None;
        }
    }
}

#[derive(Debug)]
pub struct DBState {
    populate_cancel_tx: Option<watch::Sender<bool>>,
}
impl Default for DBState {
    fn default() -> Self {
        Self::new()
    }
}
impl DBState {
    pub fn new() -> Self {
        return DBState {
            populate_cancel_tx: None,
        };
    }
    pub fn begin_populate(&mut self, tx: watch::Sender<bool>) -> Result<(), error::Error> {
        self.populate_cancel_tx = Some(tx);
        Ok(())
    }
    pub fn is_populating(&self) -> bool {
        self.populate_cancel_tx.is_some()
    }
    pub fn cancel_populate(&self) -> Result<(), error::Error> {
        if let Some(tx) = &self.populate_cancel_tx {
            let _ = tx.send(true);
        }
        Ok(())
    }
}
