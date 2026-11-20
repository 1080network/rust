// @generated
/// Generated client implementations.
pub mod issuer_administration_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct IssuerAdministrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerAdministrationServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerAdministrationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
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
        ) -> IssuerAdministrationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            IssuerAdministrationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn generate_from_mica_client_certificate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GenerateFromMicaClientCertificateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GenerateFromMicaClientCertificateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/GenerateFromMicaClientCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "GenerateFromMicaClientCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_from_mica_client_certificate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateFromMicaClientCertificateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFromMicaClientCertificateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/UpdateFromMicaClientCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "UpdateFromMicaClientCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn enable_from_mica_client_certificate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::EnableFromMicaClientCertificateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EnableFromMicaClientCertificateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/EnableFromMicaClientCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "EnableFromMicaClientCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn search_from_mica_client_certificate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SearchFromMicaClientCertificateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SearchFromMicaClientCertificateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/SearchFromMicaClientCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "SearchFromMicaClientCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_from_mica_client_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFromMicaClientSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFromMicaClientSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/GetFromMicaClientSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "GetFromMicaClientSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_from_mica_client_callback_address(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateFromMicaClientCallbackAddressRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFromMicaClientCallbackAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/UpdateFromMicaClientCallbackAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "UpdateFromMicaClientCallbackAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn search_data_extraction_statistics(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SearchDataExtractionStatisticsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SearchDataExtractionStatisticsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/SearchDataExtractionStatistics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "SearchDataExtractionStatistics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_data_extraction_statistics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataExtractionStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataExtractionStatisticsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/GetDataExtractionStatistics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "GetDataExtractionStatistics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping_external(
            &mut self,
            request: impl tonic::IntoRequest<super::PingExternalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingExternalResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/PingExternal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "PingExternal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping_external_with_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::PingExternalWithCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingExternalWithCertificateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mica.issuer.administration.v2.IssuerAdministrationService/PingExternalWithCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.administration.v2.IssuerAdministrationService",
                        "PingExternalWithCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_administration_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerAdministrationServiceServer.
    #[async_trait]
    pub trait IssuerAdministrationService: std::marker::Send + std::marker::Sync + 'static {
        async fn generate_from_mica_client_certificate(
            &self,
            request: tonic::Request<super::GenerateFromMicaClientCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateFromMicaClientCertificateResponse>,
            tonic::Status,
        >;
        async fn update_from_mica_client_certificate(
            &self,
            request: tonic::Request<super::UpdateFromMicaClientCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFromMicaClientCertificateResponse>,
            tonic::Status,
        >;
        async fn enable_from_mica_client_certificate(
            &self,
            request: tonic::Request<super::EnableFromMicaClientCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnableFromMicaClientCertificateResponse>,
            tonic::Status,
        >;
        async fn search_from_mica_client_certificate(
            &self,
            request: tonic::Request<super::SearchFromMicaClientCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchFromMicaClientCertificateResponse>,
            tonic::Status,
        >;
        async fn get_from_mica_client_settings(
            &self,
            request: tonic::Request<super::GetFromMicaClientSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFromMicaClientSettingsResponse>,
            tonic::Status,
        >;
        async fn update_from_mica_client_callback_address(
            &self,
            request: tonic::Request<super::UpdateFromMicaClientCallbackAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFromMicaClientCallbackAddressResponse>,
            tonic::Status,
        >;
        async fn search_data_extraction_statistics(
            &self,
            request: tonic::Request<super::SearchDataExtractionStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchDataExtractionStatisticsResponse>,
            tonic::Status,
        >;
        async fn get_data_extraction_statistics(
            &self,
            request: tonic::Request<super::GetDataExtractionStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataExtractionStatisticsResponse>,
            tonic::Status,
        >;
        async fn ping_external(
            &self,
            request: tonic::Request<super::PingExternalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingExternalResponse>,
            tonic::Status,
        >;
        async fn ping_external_with_certificate(
            &self,
            request: tonic::Request<super::PingExternalWithCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingExternalWithCertificateResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IssuerAdministrationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IssuerAdministrationServiceServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for IssuerAdministrationServiceServer<T>
    where
        T: IssuerAdministrationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/GenerateFromMicaClientCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateFromMicaClientCertificateSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::GenerateFromMicaClientCertificateRequest,
                    > for GenerateFromMicaClientCertificateSvc<T> {
                        type Response = super::GenerateFromMicaClientCertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateFromMicaClientCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::generate_from_mica_client_certificate(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GenerateFromMicaClientCertificateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/UpdateFromMicaClientCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFromMicaClientCertificateSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::UpdateFromMicaClientCertificateRequest,
                    > for UpdateFromMicaClientCertificateSvc<T> {
                        type Response = super::UpdateFromMicaClientCertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateFromMicaClientCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::update_from_mica_client_certificate(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = UpdateFromMicaClientCertificateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/EnableFromMicaClientCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct EnableFromMicaClientCertificateSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::EnableFromMicaClientCertificateRequest,
                    > for EnableFromMicaClientCertificateSvc<T> {
                        type Response = super::EnableFromMicaClientCertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EnableFromMicaClientCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::enable_from_mica_client_certificate(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = EnableFromMicaClientCertificateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/SearchFromMicaClientCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct SearchFromMicaClientCertificateSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::SearchFromMicaClientCertificateRequest,
                    > for SearchFromMicaClientCertificateSvc<T> {
                        type Response = super::SearchFromMicaClientCertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SearchFromMicaClientCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::search_from_mica_client_certificate(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = SearchFromMicaClientCertificateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/GetFromMicaClientSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetFromMicaClientSettingsSvc<T: IssuerAdministrationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::GetFromMicaClientSettingsRequest,
                    > for GetFromMicaClientSettingsSvc<T> {
                        type Response = super::GetFromMicaClientSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetFromMicaClientSettingsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::get_from_mica_client_settings(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GetFromMicaClientSettingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/UpdateFromMicaClientCallbackAddress" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFromMicaClientCallbackAddressSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::UpdateFromMicaClientCallbackAddressRequest,
                    > for UpdateFromMicaClientCallbackAddressSvc<T> {
                        type Response = super::UpdateFromMicaClientCallbackAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateFromMicaClientCallbackAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::update_from_mica_client_callback_address(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = UpdateFromMicaClientCallbackAddressSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/SearchDataExtractionStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct SearchDataExtractionStatisticsSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::SearchDataExtractionStatisticsRequest,
                    > for SearchDataExtractionStatisticsSvc<T> {
                        type Response = super::SearchDataExtractionStatisticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SearchDataExtractionStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::search_data_extraction_statistics(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = SearchDataExtractionStatisticsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/GetDataExtractionStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataExtractionStatisticsSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::GetDataExtractionStatisticsRequest,
                    > for GetDataExtractionStatisticsSvc<T> {
                        type Response = super::GetDataExtractionStatisticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDataExtractionStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::get_data_extraction_statistics(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GetDataExtractionStatisticsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/PingExternal" => {
                    #[allow(non_camel_case_types)]
                    struct PingExternalSvc<T: IssuerAdministrationService>(pub Arc<T>);
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<super::PingExternalRequest>
                    for PingExternalSvc<T> {
                        type Response = super::PingExternalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingExternalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::ping_external(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = PingExternalSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/mica.issuer.administration.v2.IssuerAdministrationService/PingExternalWithCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct PingExternalWithCertificateSvc<
                        T: IssuerAdministrationService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerAdministrationService,
                    > tonic::server::UnaryService<
                        super::PingExternalWithCertificateRequest,
                    > for PingExternalWithCertificateSvc<T> {
                        type Response = super::PingExternalWithCertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PingExternalWithCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerAdministrationService>::ping_external_with_certificate(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = PingExternalWithCertificateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for IssuerAdministrationServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "mica.issuer.administration.v2.IssuerAdministrationService";
    impl<T> tonic::server::NamedService for IssuerAdministrationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
