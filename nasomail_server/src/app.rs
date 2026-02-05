use ctxguard::tokio::ContextGuard;
use sqlx::sqlite::SqlitePool;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::config::Config;

pub type AppContextGuard = ContextGuard<AppContext>;

pub struct AppContext {
    pool: RwLock<SqlitePool>,
    cfg: RwLock<Config>,

    test_code: RwLock<String>,
}

impl AppContext {
    pub fn new(pool: SqlitePool, cfg: Config) -> AppContextGuard {
        ContextGuard::new(Self {
            pool: RwLock::new(pool),
            cfg: RwLock::new(cfg),

            test_code: RwLock::new(Uuid::new_v4().to_string()),
        })
    }

    pub async fn pool(&self) -> RwLockReadGuard<'_, SqlitePool> {
        self.pool.read().await
    }
    pub async fn pool_mut(&self) -> RwLockWriteGuard<'_, SqlitePool> {
        self.pool.write().await
    }

    pub async fn cfg(&self) -> RwLockReadGuard<'_, Config> {
        self.cfg.read().await
    }
    pub async fn cfg_mut(&self) -> RwLockWriteGuard<'_, Config> {
        self.cfg.write().await
    }

    pub async fn test_code(&self) -> RwLockReadGuard<'_, String> {
        self.test_code.read().await
    }
}
