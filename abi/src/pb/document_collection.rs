#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<::prost_wkt_types::Struct>,
    #[prost(message, optional, tag = "4")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentQuery {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<DocumentQuery>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<::prost_wkt_types::Struct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<::prost_wkt_types::Struct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
/// Generated client implementations.
pub mod document_collection_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DocumentCollectionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentCollectionClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DocumentCollectionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DocumentCollectionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DocumentCollectionClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/document_collection.DocumentCollection/get");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "document_collection.DocumentCollection",
                "get",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Document>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/document_collection.DocumentCollection/query",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "document_collection.DocumentCollection",
                "query",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/document_collection.DocumentCollection/create",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "document_collection.DocumentCollection",
                "create",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/document_collection.DocumentCollection/update",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "document_collection.DocumentCollection",
                "update",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/document_collection.DocumentCollection/delete",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "document_collection.DocumentCollection",
                "delete",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod document_collection_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DocumentCollectionServer.
    #[async_trait]
    pub trait DocumentCollection: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status>;
        /// Server streaming response type for the query method.
        type queryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Document, tonic::Status>,
            > + Send
            + 'static;
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::queryStream>, tonic::Status>;
        async fn create(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status>;
        async fn update(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DocumentCollectionServer<T: DocumentCollection> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DocumentCollection> DocumentCollectionServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DocumentCollectionServer<T>
    where
        T: DocumentCollection,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/document_collection.DocumentCollection/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: DocumentCollection>(pub Arc<T>);
                    impl<T: DocumentCollection> tonic::server::UnaryService<super::GetRequest> for getSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DocumentCollection>::get(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/document_collection.DocumentCollection/query" => {
                    #[allow(non_camel_case_types)]
                    struct querySvc<T: DocumentCollection>(pub Arc<T>);
                    impl<T: DocumentCollection>
                        tonic::server::ServerStreamingService<super::QueryRequest> for querySvc<T>
                    {
                        type Response = super::Document;
                        type ResponseStream = T::queryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DocumentCollection>::query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = querySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/document_collection.DocumentCollection/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: DocumentCollection>(pub Arc<T>);
                    impl<T: DocumentCollection> tonic::server::UnaryService<super::CreateRequest> for createSvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DocumentCollection>::create(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/document_collection.DocumentCollection/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: DocumentCollection>(pub Arc<T>);
                    impl<T: DocumentCollection> tonic::server::UnaryService<super::UpdateRequest> for updateSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DocumentCollection>::update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/document_collection.DocumentCollection/delete" => {
                    #[allow(non_camel_case_types)]
                    struct deleteSvc<T: DocumentCollection>(pub Arc<T>);
                    impl<T: DocumentCollection> tonic::server::UnaryService<super::DeleteRequest> for deleteSvc<T> {
                        type Response = super::DeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DocumentCollection>::delete(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DocumentCollection> Clone for DocumentCollectionServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DocumentCollection> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DocumentCollection> tonic::server::NamedService for DocumentCollectionServer<T> {
        const NAME: &'static str = "document_collection.DocumentCollection";
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DOCUMENT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.Document")]
    impl ::prost_wkt::MessageSerde for Document {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "Document"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.Document"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.Document" , decoder : | buf : & [u8] | { let msg : Document = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for Document {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "Document";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.Document".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.GetRequest")]
    impl ::prost_wkt::MessageSerde for GetRequest {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "GetRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.GetRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.GetRequest" , decoder : | buf : & [u8] | { let msg : GetRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for GetRequest {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "GetRequest";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.GetRequest".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.GetResponse")]
    impl ::prost_wkt::MessageSerde for GetResponse {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "GetResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.GetResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.GetResponse" , decoder : | buf : & [u8] | { let msg : GetResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for GetResponse {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "GetResponse";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.GetResponse".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DOCUMENT_QUERY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.DocumentQuery")]
    impl ::prost_wkt::MessageSerde for DocumentQuery {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "DocumentQuery"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.DocumentQuery"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.DocumentQuery" , decoder : | buf : & [u8] | { let msg : DocumentQuery = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for DocumentQuery {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "DocumentQuery";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.DocumentQuery".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.QueryRequest")]
    impl ::prost_wkt::MessageSerde for QueryRequest {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "QueryRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.QueryRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.QueryRequest" , decoder : | buf : & [u8] | { let msg : QueryRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for QueryRequest {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "QueryRequest";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.QueryRequest".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CREATE_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.CreateRequest")]
    impl ::prost_wkt::MessageSerde for CreateRequest {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "CreateRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.CreateRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.CreateRequest" , decoder : | buf : & [u8] | { let msg : CreateRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for CreateRequest {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "CreateRequest";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.CreateRequest".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CREATE_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.CreateResponse")]
    impl ::prost_wkt::MessageSerde for CreateResponse {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "CreateResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.CreateResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.CreateResponse" , decoder : | buf : & [u8] | { let msg : CreateResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for CreateResponse {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "CreateResponse";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.CreateResponse".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_UPDATE_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.UpdateRequest")]
    impl ::prost_wkt::MessageSerde for UpdateRequest {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "UpdateRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.UpdateRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.UpdateRequest" , decoder : | buf : & [u8] | { let msg : UpdateRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for UpdateRequest {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "UpdateRequest";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.UpdateRequest".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_UPDATE_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.UpdateResponse")]
    impl ::prost_wkt::MessageSerde for UpdateResponse {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "UpdateResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.UpdateResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.UpdateResponse" , decoder : | buf : & [u8] | { let msg : UpdateResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for UpdateResponse {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "UpdateResponse";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.UpdateResponse".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELETE_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.DeleteRequest")]
    impl ::prost_wkt::MessageSerde for DeleteRequest {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "DeleteRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.DeleteRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.DeleteRequest" , decoder : | buf : & [u8] | { let msg : DeleteRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for DeleteRequest {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "DeleteRequest";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.DeleteRequest".to_string()
        }
    }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELETE_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/document_collection.DeleteResponse")]
    impl ::prost_wkt::MessageSerde for DeleteResponse {
        fn package_name(&self) -> &'static str {
            "document_collection"
        }
        fn message_name(&self) -> &'static str {
            "DeleteResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/document_collection.DeleteResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/document_collection.DeleteResponse" , decoder : | buf : & [u8] | { let msg : DeleteResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
    impl ::prost::Name for DeleteResponse {
        const PACKAGE: &'static str = "document_collection";
        const NAME: &'static str = "DeleteResponse";
        fn type_url() -> String {
            "type.googleapis.com/document_collection.DeleteResponse".to_string()
        }
    }
};
