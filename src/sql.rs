use anyhow::{Ok, Result};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Sqlite, SqlitePool};
use std::time::Duration;

use crate::file::FileObj;

pub struct Client {
    pool: SqlitePool,
}

impl Client {
    pub async fn new(file_name: &str) -> Result<Self> {
        let db_url = format!("sqlite://{}", file_name);

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            match Sqlite::create_database(&db_url).await {
                // Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
                _ => (),
            }
        }

        let c = Client {
            pool: SqlitePool::connect(&db_url).await?,
        };

        c.setup().await?;

        Ok(c)
    }

    /// Initialised tables and indexes
    async fn setup(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                    path TEXT UNIQUE ON CONFLICT REPLACE, 
                    ext TEXT, 
                    bytes INTEGER NOT NULL, 
                    created INTEGER NOT NULL, 
                    modified INTEGER NOT NULL
                );",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS path_idx ON files (path);")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn insert(&self, fo: &FileObj) -> Result<SqliteQueryResult> {
        let result = sqlx::query(
            "INSERT INTO files (path, ext, bytes, created, modified) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&fo.path)
        .bind(&fo.ext)
        .bind(fo.bytes)
        .bind(fo.created.as_secs() as i64)
        .bind(fo.modified.as_secs() as i64)
        .execute(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        // HACK: Need the following sleep otherwise we end up
        // with `.db-wal` and `.db-shm` files alongside our main `.db` file?
        std::thread::sleep(Duration::from_millis(1));
        Ok(())
    }
}
