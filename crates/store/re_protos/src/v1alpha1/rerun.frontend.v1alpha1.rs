// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterPartitionsRequest {
    #[prost(message, optional, tag = "1")]
    pub dataset_id: ::core::option::Option<super::super::common::v1alpha1::Tuid>,
    /// Partitions to add
    #[prost(message, repeated, tag = "2")]
    pub partitions: ::prost::alloc::vec::Vec<super::super::manifest_registry::v1alpha1::Partition>,
    /// what to do if partition is already registered
    /// TODO(cmc): why is this in manifest_registry??
    #[prost(
        enumeration = "super::super::manifest_registry::v1alpha1::IfDuplicateBehavior",
        tag = "3"
    )]
    pub on_duplicate: i32,
}
impl ::prost::Name for RegisterPartitionsRequest {
    const NAME: &'static str = "RegisterPartitionsRequest";
    const PACKAGE: &'static str = "rerun.frontend.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        "rerun.frontend.v1alpha1.RegisterPartitionsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/rerun.frontend.v1alpha1.RegisterPartitionsRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterPartitionsRequest {
    #[prost(message, optional, tag = "1")]
    pub dataset_id: ::core::option::Option<super::super::common::v1alpha1::Tuid>,
    /// Partitions to remove
    #[prost(message, repeated, tag = "2")]
    pub partition_ids: ::prost::alloc::vec::Vec<super::super::common::v1alpha1::PartitionId>,
    /// What to do if partition is not found
    #[prost(
        enumeration = "super::super::common::v1alpha1::IfMissingBehavior",
        tag = "3"
    )]
    pub on_unknown_partition: i32,
}
impl ::prost::Name for UnregisterPartitionsRequest {
    const NAME: &'static str = "UnregisterPartitionsRequest";
    const PACKAGE: &'static str = "rerun.frontend.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        "rerun.frontend.v1alpha1.UnregisterPartitionsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/rerun.frontend.v1alpha1.UnregisterPartitionsRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionsRequest {
    #[prost(message, optional, tag = "1")]
    pub dataset_id: ::core::option::Option<super::super::common::v1alpha1::Tuid>,
    /// Scan parameters
    #[prost(message, optional, tag = "2")]
    pub scan_parameters: ::core::option::Option<super::super::common::v1alpha1::ScanParameters>,
}
impl ::prost::Name for ListPartitionsRequest {
    const NAME: &'static str = "ListPartitionsRequest";
    const PACKAGE: &'static str = "rerun.frontend.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        "rerun.frontend.v1alpha1.ListPartitionsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/rerun.frontend.v1alpha1.ListPartitionsRequest".into()
    }
}
/// Generated client implementations.
pub mod frontend_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Redap's public API.
    #[derive(Debug, Clone)]
    pub struct FrontendServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FrontendServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> FrontendServiceClient<InterceptedService<T, F>>
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
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            FrontendServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn find_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::catalog::v1alpha1::FindEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::FindEntriesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/FindEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "FindEntries",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_dataset_entry(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::catalog::v1alpha1::CreateDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::CreateDatasetEntryResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/CreateDatasetEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "CreateDatasetEntry",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_dataset_entry(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::catalog::v1alpha1::ReadDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::ReadDatasetEntryResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/ReadDatasetEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "ReadDatasetEntry",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_dataset_entry(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::catalog::v1alpha1::DeleteDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::DeleteDatasetEntryResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/DeleteDatasetEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "DeleteDatasetEntry",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Register new partitions with the Dataset
        pub async fn register_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::manifest_registry::v1alpha1::RegisterPartitionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/RegisterPartitions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "RegisterPartitions",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Unregister partitions from the Dataset
        pub async fn unregister_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::UnregisterPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::manifest_registry::v1alpha1::UnregisterPartitionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/UnregisterPartitions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "UnregisterPartitions",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// List partitions in the Dataset
        pub async fn list_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::manifest_registry::v1alpha1::ListPartitionsResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rerun.frontend.v1alpha1.FrontendService/ListPartitions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "rerun.frontend.v1alpha1.FrontendService",
                "ListPartitions",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod frontend_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FrontendServiceServer.
    #[async_trait]
    pub trait FrontendService: std::marker::Send + std::marker::Sync + 'static {
        async fn find_entries(
            &self,
            request: tonic::Request<super::super::super::catalog::v1alpha1::FindEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::FindEntriesResponse>,
            tonic::Status,
        >;
        async fn create_dataset_entry(
            &self,
            request: tonic::Request<
                super::super::super::catalog::v1alpha1::CreateDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::CreateDatasetEntryResponse>,
            tonic::Status,
        >;
        async fn read_dataset_entry(
            &self,
            request: tonic::Request<
                super::super::super::catalog::v1alpha1::ReadDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::ReadDatasetEntryResponse>,
            tonic::Status,
        >;
        async fn delete_dataset_entry(
            &self,
            request: tonic::Request<
                super::super::super::catalog::v1alpha1::DeleteDatasetEntryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::catalog::v1alpha1::DeleteDatasetEntryResponse>,
            tonic::Status,
        >;
        /// Register new partitions with the Dataset
        async fn register_partitions(
            &self,
            request: tonic::Request<super::RegisterPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::manifest_registry::v1alpha1::RegisterPartitionsResponse,
            >,
            tonic::Status,
        >;
        /// Unregister partitions from the Dataset
        async fn unregister_partitions(
            &self,
            request: tonic::Request<super::UnregisterPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::manifest_registry::v1alpha1::UnregisterPartitionsResponse,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the ListPartitions method.
        type ListPartitionsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::manifest_registry::v1alpha1::ListPartitionsResponse,
                    tonic::Status,
                >,
            > + std::marker::Send
            + 'static;
        /// List partitions in the Dataset
        async fn list_partitions(
            &self,
            request: tonic::Request<super::ListPartitionsRequest>,
        ) -> std::result::Result<tonic::Response<Self::ListPartitionsStream>, tonic::Status>;
    }
    /// Redap's public API.
    #[derive(Debug)]
    pub struct FrontendServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> FrontendServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FrontendServiceServer<T>
    where
        T: FrontendService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
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
            match req.uri().path() {
                "/rerun.frontend.v1alpha1.FrontendService/FindEntries" => {
                    #[allow(non_camel_case_types)]
                    struct FindEntriesSvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<
                            super::super::super::catalog::v1alpha1::FindEntriesRequest,
                        > for FindEntriesSvc<T>
                    {
                        type Response = super::super::super::catalog::v1alpha1::FindEntriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::catalog::v1alpha1::FindEntriesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::find_entries(&inner, request).await
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
                        let method = FindEntriesSvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/CreateDatasetEntry" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDatasetEntrySvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<
                            super::super::super::catalog::v1alpha1::CreateDatasetEntryRequest,
                        > for CreateDatasetEntrySvc<T>
                    {
                        type Response =
                            super::super::super::catalog::v1alpha1::CreateDatasetEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::catalog::v1alpha1::CreateDatasetEntryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::create_dataset_entry(&inner, request).await
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
                        let method = CreateDatasetEntrySvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/ReadDatasetEntry" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDatasetEntrySvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<
                            super::super::super::catalog::v1alpha1::ReadDatasetEntryRequest,
                        > for ReadDatasetEntrySvc<T>
                    {
                        type Response =
                            super::super::super::catalog::v1alpha1::ReadDatasetEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::catalog::v1alpha1::ReadDatasetEntryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::read_dataset_entry(&inner, request).await
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
                        let method = ReadDatasetEntrySvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/DeleteDatasetEntry" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetEntrySvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<
                            super::super::super::catalog::v1alpha1::DeleteDatasetEntryRequest,
                        > for DeleteDatasetEntrySvc<T>
                    {
                        type Response =
                            super::super::super::catalog::v1alpha1::DeleteDatasetEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::catalog::v1alpha1::DeleteDatasetEntryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::delete_dataset_entry(&inner, request).await
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
                        let method = DeleteDatasetEntrySvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/RegisterPartitions" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterPartitionsSvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<super::RegisterPartitionsRequest>
                        for RegisterPartitionsSvc<T>
                    {
                        type Response = super::super::super::manifest_registry::v1alpha1::RegisterPartitionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterPartitionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::register_partitions(&inner, request).await
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
                        let method = RegisterPartitionsSvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/UnregisterPartitions" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterPartitionsSvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::UnaryService<super::UnregisterPartitionsRequest>
                        for UnregisterPartitionsSvc<T>
                    {
                        type Response = super::super::super::manifest_registry::v1alpha1::UnregisterPartitionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnregisterPartitionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::unregister_partitions(&inner, request).await
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
                        let method = UnregisterPartitionsSvc(inner);
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
                "/rerun.frontend.v1alpha1.FrontendService/ListPartitions" => {
                    #[allow(non_camel_case_types)]
                    struct ListPartitionsSvc<T: FrontendService>(pub Arc<T>);
                    impl<T: FrontendService>
                        tonic::server::ServerStreamingService<super::ListPartitionsRequest>
                        for ListPartitionsSvc<T>
                    {
                        type Response = super::super::super::manifest_registry::v1alpha1::ListPartitionsResponse;
                        type ResponseStream = T::ListPartitionsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPartitionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FrontendService>::list_partitions(&inner, request).await
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
                        let method = ListPartitionsSvc(inner);
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
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for FrontendServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "rerun.frontend.v1alpha1.FrontendService";
    impl<T> tonic::server::NamedService for FrontendServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
