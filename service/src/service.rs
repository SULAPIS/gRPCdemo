use std::{pin::Pin, task::Poll};

use abi::{
    document_collection_server::DocumentCollection, Config, CreateRequest, CreateResponse,
    DeleteRequest, DeleteResponse, DocumentQuery, GetRequest, GetResponse, QueryRequest,
    UpdateRequest, UpdateResponse,
};
use document_collection::{Dc, DcManager};
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{async_trait, Request, Response, Status};

use crate::{DcService, DocumentStream, TonicReceiverStream};

impl DcService {
    pub async fn from_config(config: &Config) -> Result<Self, anyhow::Error> {
        Ok(Self {
            manager: DcManager::from_config(&config.db).await?,
        })
    }
}

#[async_trait]
impl DocumentCollection for DcService {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let request = request.into_inner();
        if let (user_id, Some(data)) = (request.user_id, request.data) {
            let document = self.manager.create(user_id, data).await?;
            Ok(Response::new(CreateResponse {
                document: Some(document),
            }))
        } else {
            Err(Status::invalid_argument("missing data"))
        }
    }

    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request = request.into_inner();
        if let (id, Some(data)) = (request.id, request.data) {
            let document = self.manager.update(id, data).await?;

            Ok(Response::new(UpdateResponse {
                document: Some(document),
            }))
        } else {
            Err(Status::invalid_argument("missing data"))
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let request = request.into_inner();
        let document = self.manager.delete(request.id).await?;

        Ok(Response::new(DeleteResponse {
            document: Some(document),
        }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let document = self.manager.get(request.id).await?;

        Ok(Response::new(GetResponse {
            document: Some(document),
        }))
    }

    type queryStream = DocumentStream;
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::queryStream>, Status> {
        let request = request.into_inner();
        if let Some(request) = request.query {
            let docs = self.manager.query(request).await;

            let stream = TonicReceiverStream::new(docs);
            Ok(Response::new(Box::pin(stream) as Self::queryStream))
        } else {
            Err(Status::invalid_argument("missing query"))
        }
    }
}

impl<T> TonicReceiverStream<T> {
    pub fn new(inner: mpsc::Receiver<Result<T, abi::Error>>) -> Self {
        Self { inner }
    }
}

impl<T> Stream for TonicReceiverStream<T> {
    type Item = Result<T, Status>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.inner.poll_recv(cx) {
            Poll::Ready(Some(Ok(item))) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
