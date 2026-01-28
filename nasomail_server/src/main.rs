use std::path::{Path, PathBuf};
use tracing::{Instrument, info, info_span, instrument};

use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub mod app;
pub mod config;
pub mod meta;

use crate::{app::*, config::Config};

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let span = info_span!("startup");

    let (listener, router) = async {
        let span = info_span!("config");
        let cfg = async {
            let cfg_path = Path::new(meta::CONFIG_PATH);

            match fs::try_exists(cfg_path).await {
                Ok(true) => {
                    info!(cfg_path = ?cfg_path, "reading config");

                    let cfg_json = fs::read_to_string(cfg_path)
                        .await
                        .expect("failed to read config file");

                    serde_json::from_str(&cfg_json).expect("failed to deserialize config file")
                }
                Ok(false) => {
                    info!(cfg_path = ?cfg_path, "creating config");

                    let cfg = Config::default();
                    if let Some(parent) = cfg_path.parent()
                        && !match fs::try_exists(parent).await {
                            Ok(true) => true,
                            Ok(false) => false,
                            Err(e) => {
                                panic!("failed to check if config parent directory exists: {e}")
                            }
                        }
                    {
                        fs::create_dir_all(parent)
                            .await
                            .expect("failed to create config parent directories");
                    }

                    let cfg_json = serde_json::to_string_pretty(&cfg)
                        .expect("failed to serialize config file");

                    let mut file = File::create(cfg_path)
                        .await
                        .expect("failed to create/open config file");

                    file.write_all(cfg_json.as_bytes())
                        .await
                        .expect("failed to write config file");

                    cfg
                }
                Err(e) => panic!("failed to check if config file exists: {e}"),
            }
        }
        .instrument(span)
        .await;

        let span = info_span!("database");
        let db = async {
            let db_path = PathBuf::from(&cfg.db_path);
            match fs::try_exists(&db_path).await {
                Ok(false) => {
                    info!(db_path = ?db_path, "creating database");

                    if let Some(parent) = db_path.parent()
                        && match fs::try_exists(&parent).await {
                            Ok(exists) => exists,
                            Err(e) => {
                                panic!("failed to check if database parent directory exists: {e}")
                            }
                        }
                    {
                        fs::create_dir_all(parent)
                            .await
                            .expect("failed to create database parent directories");
                    }

                    File::create(db_path)
                        .await
                        .expect("failed to create database file");
                }
                Err(e) => panic!("failed to check if database file exists: {e}"),
                _ => {}
            };

            let url = format!("sqlite://{}", cfg.db_path);
            info!(url = %url, "connecting to database");

            SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .expect("failed to connect to database")
        }
        .instrument(span)
        .await;

        let app = AppContext::new(db, cfg);

        let app_ctx = app.ctx().await;
        let cfg = app_ctx.cfg().await;

        let router = Router::new().with_state(app.clone());

        let listener = tokio::net::TcpListener::bind(&cfg.addr)
            .await
            .expect("failed to bind TCP listener");

        info!(addr = %cfg.addr, "listening");

        (listener, router)
    }
    .instrument(span)
    .await;

    axum::serve(listener, router)
        .await
        .expect("failed to start server");
}
