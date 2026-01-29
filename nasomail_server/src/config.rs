use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct Config {
    db_path: RwLock<String>,
    schema_path: RwLock<String>,

    addr: RwLock<String>,
    pub_addr: RwLock<String>,
}

impl Config {
    pub fn new(db_path: String, schema_path: String, addr: String, pub_addr: String) -> Self {
        Self {
            db_path: RwLock::new(db_path),
            schema_path: RwLock::new(schema_path),

            addr: RwLock::new(addr),
            pub_addr: RwLock::new(pub_addr),
        }
    }

    pub async fn to_ser(&self) -> ConfigSerializable {
        ConfigSerializable {
            db_path: self.db_path.read().await.clone(),
            schema_path: self.schema_path.read().await.clone(),

            addr: self.addr.read().await.clone(),
            pub_addr: self.pub_addr.read().await.clone(),
        }
    }

    pub async fn db_path(&self) -> RwLockReadGuard<'_, String> {
        self.db_path.read().await
    }
    pub async fn db_path_mut(&self) -> RwLockWriteGuard<'_, String> {
        self.db_path.write().await
    }
    pub async fn set_db_path(&mut self, value: String) {
        self.db_path = RwLock::new(value);
    }

    pub async fn schema_path(&self) -> RwLockReadGuard<'_, String> {
        self.schema_path.read().await
    }
    pub async fn schema_path_mut(&self) -> RwLockWriteGuard<'_, String> {
        self.schema_path.write().await
    }
    pub async fn set_schema_path(&mut self, value: String) {
        self.schema_path = RwLock::new(value);
    }

    pub async fn addr(&self) -> RwLockReadGuard<'_, String> {
        self.addr.read().await
    }
    pub async fn addr_mut(&self) -> RwLockWriteGuard<'_, String> {
        self.addr.write().await
    }
    pub async fn set_addr(&mut self, value: String) {
        self.addr = RwLock::new(value);
    }

    pub async fn pub_addr(&self) -> RwLockReadGuard<'_, String> {
        self.pub_addr.read().await
    }
    pub async fn pub_addr_mut(&self) -> RwLockWriteGuard<'_, String> {
        self.pub_addr.write().await
    }
    pub async fn set_pub_addr(&mut self, value: String) {
        self.pub_addr = RwLock::new(value);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: RwLock::new("database.sqlite".to_owned()),
            schema_path: RwLock::new("sql/schema.sql".to_owned()),

            addr: RwLock::new("0.0.0.0:8080".to_owned()),
            pub_addr: RwLock::new("mail.example.com:8080".to_owned()),
        }
    }
}

impl From<ConfigSerializable> for Config {
    fn from(value: ConfigSerializable) -> Self {
        Self {
            db_path: RwLock::new(value.db_path),
            schema_path: RwLock::new(value.schema_path),

            addr: RwLock::new(value.addr),
            pub_addr: RwLock::new(value.pub_addr),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSerializable {
    pub db_path: String,
    pub schema_path: String,

    pub addr: String,
    pub pub_addr: String,
}

impl ConfigSerializable {
    pub async fn from_cfg(value: &Config) -> Self {
        Self {
            db_path: value.db_path().await.clone(),
            schema_path: value.schema_path().await.clone(),

            addr: value.addr().await.clone(),
            pub_addr: value.pub_addr().await.clone(),
        }
    }
}
