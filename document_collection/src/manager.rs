use std::time::SystemTime;

use abi::DbConfig;
use async_trait::async_trait;
use chrono::{DateTime, Days, Utc};
use futures::StreamExt;
use prost_wkt_types::Struct;
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, types::Uuid, Either, PgPool};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::{Dc, DcManager};

#[async_trait]
impl Dc for DcManager {
    async fn create(&self, user_id: String, data: Struct) -> Result<abi::Document, abi::Error> {
        let data = serde_json::to_value(data).unwrap();
        let user_id = Uuid::parse_str(&user_id).unwrap();
        let document: abi::Document =
            sqlx::query_as("INSERT INTO dc.documents (user_id, data) VALUES ($1, $2) RETURNING *")
                .bind(user_id)
                .bind(data)
                .fetch_one(&self.pool)
                .await?;

        Ok(document)
    }

    async fn update(&self, id: abi::DocumentId, data: Struct) -> Result<abi::Document, abi::Error> {
        info!("Updating document: {:?}", data);
        let data = serde_json::to_value(data).unwrap();
        let id = Uuid::parse_str(&id).unwrap();
        let document: abi::Document =
            sqlx::query_as("UPDATE dc.documents SET data = $1 WHERE id = $2 RETURNING *")
                .bind(data)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(document)
    }

    async fn delete(&self, id: abi::DocumentId) -> Result<abi::Document, abi::Error> {
        let id = Uuid::parse_str(&id).unwrap();
        let document: abi::Document =
            sqlx::query_as("DELETE FROM dc.documents WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(document)
    }

    async fn get(&self, id: abi::DocumentId) -> Result<abi::Document, abi::Error> {
        let id = Uuid::parse_str(&id).unwrap();
        let document: abi::Document = sqlx::query_as("SELECT * FROM dc.documents WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(document)
    }

    async fn query(
        &self,
        query: abi::DocumentQuery,
    ) -> mpsc::Receiver<Result<abi::Document, abi::Error>> {
        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let sql = "SELECT * FROM dc.documents WHERE user_id = $1 AND created_at >= $2 AND created_at < $3";
            let mut docs = sqlx::query_as(&sql).fetch_many(&pool);
            while let Some(ret) = docs.next().await {
                match ret {
                    Ok(Either::Left(r)) => {
                        info!("Query result: {:?}", r);
                    }
                    Ok(Either::Right(r)) => {
                        if tx.send(Ok(r)).await.is_err() {
                            // rx is dropped, so client disconnected
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("Query error: {:?}", e);
                        if tx.send(Err(e.into())).await.is_err() {
                            // rx is dropped, so client disconnected
                            break;
                        }
                    }
                }
            }
        });
        rx
    }
}

impl DcManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_config(config: &DbConfig) -> Result<Self, abi::Error> {
        let url = config.url();
        let pool = PgPoolOptions::default()
            .max_connections(config.max_connections)
            .connect(&url)
            .await?;
        Ok(Self::new(pool))
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgRow, PgPool, Row};

    #[sqlx::test]
    async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let foo: PgRow = sqlx::query("SELECT * FROM foo")
            .fetch_one(conn.as_mut())
            .await?;

        assert_eq!(foo.get::<String, _>("bar"), "foobar!");

        Ok(())
    }
}
