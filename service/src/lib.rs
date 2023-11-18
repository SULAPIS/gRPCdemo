mod service;

use std::pin::Pin;

use abi::{document_collection_server::DocumentCollectionServer, Config, Document};
use document_collection::DcManager;
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{transport::Server, Status};

pub struct DcService {
    manager: DcManager,
}

pub struct TonicReceiverStream<T> {
    inner: mpsc::Receiver<Result<T, abi::Error>>,
}

type DocumentStream = Pin<Box<dyn Stream<Item = Result<Document, Status>> + Send>>;

pub async fn start_server(config: &Config) -> Result<(), anyhow::Error> {
    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    let svc = DcService::from_config(config).await?;
    let svc = DocumentCollectionServer::new(svc);

    println!("Listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
