#![allow(dead_code)]
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppState {
    pool: SqlitePool,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
