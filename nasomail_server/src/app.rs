use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::config::Config;

pub type AppContextGuardPtr = Arc<AppContextGuard>;

pub struct AppContextGuard {
    ctx: RwLock<AppContext>,
}

impl AppContextGuard {
    pub fn new(ctx: AppContext) -> AppContextGuardPtr {
        Arc::new(Self {
            ctx: RwLock::new(ctx),
        })
    }

    pub async fn ctx(&self) -> RwLockReadGuard<'_, AppContext> {
        self.ctx.read().await
    }
}

pub struct AppContext {
    pool: RwLock<SqlitePool>,
    cfg: RwLock<Config>,
}

impl AppContext {
    pub fn new(pool: SqlitePool, cfg: Config) -> AppContextGuardPtr {
        AppContextGuard::new(Self {
            pool: RwLock::new(pool),
            cfg: RwLock::new(cfg),
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
}
