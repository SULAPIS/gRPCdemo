mod manager;

use async_trait::async_trait;
use prost_wkt_types::Struct;
use serde_json::Value;
use sqlx::PgPool;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct DcManager {
    pool: PgPool,
}

#[async_trait]
pub trait Dc {
    /// Create a new document.
    async fn create(&self, user_id: String, data: Struct) -> Result<abi::Document, abi::Error>;
    /// Update a document.
    async fn update(&self, id: abi::DocumentId, data: Struct) -> Result<abi::Document, abi::Error>;
    /// Delete a document.
    async fn delete(&self, id: abi::DocumentId) -> Result<abi::Document, abi::Error>;
    /// Get a document.
    async fn get(&self, id: abi::DocumentId) -> Result<abi::Document, abi::Error>;
    /// query
    async fn query(
        &self,
        query: abi::DocumentQuery,
    ) -> mpsc::Receiver<Result<abi::Document, abi::Error>>;
}
