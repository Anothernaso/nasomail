use anyhow::anyhow;
use std::path::{Path, PathBuf};
use tracing::{info, instrument};

use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
    time::{self, Duration},
};

mod api;
mod app;
mod config;
mod meta;

use crate::{
    api::{RouterApi, ctest},
    app::*,
    config::{Config, ConfigSerializable},
};

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // ############################
    // ## Load the configuration ##
    // ############################

    let cfg_path = Path::new(meta::CONFIG_PATH);

    let cfg = if fs::try_exists(cfg_path).await? {
        info!(cfg_path = ?cfg_path, "reading config");

        let cfg_json = fs::read_to_string(cfg_path).await?;

        Config::from(serde_json::from_str::<ConfigSerializable>(&cfg_json)?)
    } else {
        info!(cfg_path = ?cfg_path, "creating config");

        let cfg = Config::default();
        if let Some(parent) = cfg_path.parent()
            && !fs::try_exists(parent).await?
        {
            fs::create_dir_all(parent).await?;
        }

        let cfg_json = serde_json::to_string_pretty(&cfg.to_ser().await)?;

        let mut file = File::create(cfg_path).await?;

        file.write_all(cfg_json.as_bytes()).await?;

        cfg
    };

    // #############################
    // ## Initialize the database ##
    // #############################

    let db_path = PathBuf::from(cfg.db_path().await.clone());
    if !fs::try_exists(&db_path).await? {
        info!(db_path = ?db_path, "creating database");

        if let Some(parent) = db_path.parent()
            && !fs::try_exists(&parent).await?
        {
            fs::create_dir_all(parent).await?;
        }

        File::create(db_path).await?;
    }

    let url = format!("sqlite://{}", cfg.db_path().await);
    info!(url = %url, "connecting to database");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let schema_path = PathBuf::from(cfg.schema_path().await.clone());

    if fs::try_exists(&schema_path).await? {
        info!(schema_path = ?schema_path, "executing schema");

        let schema = fs::read_to_string(schema_path).await?;

        for stmt in schema.split(';').filter(|s| !s.trim().is_empty()) {
            sqlx::query(stmt).execute(&pool).await?;
        }
    } else {
        return Err(anyhow!(format!(
            "schema file does not exist: {}",
            schema_path.display()
        )));
    }

    // ####################
    // ## Run the server ##
    // ####################

    let app = AppContext::new(pool, cfg);

    let ctx = app.ctx().await;
    let cfg = ctx.cfg().await;

    let router = Router::new().with_api().with_state(app.clone());

    let listener = tokio::net::TcpListener::bind(cfg.addr().await.clone()).await?;

    info!(addr = %cfg.addr().await, "listening");
    let handle = tokio::spawn(async move { axum::serve(listener, router).await });

    time::sleep(Duration::from_secs(1)).await;
    ctest::connection_test(app.clone()).await;
    handle.await??;

    Ok(())
}
