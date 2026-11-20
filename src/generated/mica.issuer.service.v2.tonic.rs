// @generated
/// Generated client implementations.
pub mod issuer_from_mica_service_client {
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
    pub struct IssuerFromMicaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerFromMicaServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerFromMicaServiceClient<T>
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
        ) -> IssuerFromMicaServiceClient<InterceptedService<T, F>>
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
            IssuerFromMicaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn obtain_value(
            &mut self,
            request: impl tonic::IntoRequest<super::ObtainValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ObtainValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ObtainValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "ObtainValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reverse_obtain_value(
            &mut self,
            request: impl tonic::IntoRequest<super::ReverseObtainValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseObtainValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ReverseObtainValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "ReverseObtainValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn refund_value(
            &mut self,
            request: impl tonic::IntoRequest<super::RefundValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefundValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/RefundValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "RefundValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn payout_value(
            &mut self,
            request: impl tonic::IntoRequest<super::PayoutValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PayoutValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/PayoutValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "PayoutValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn hold_value(
            &mut self,
            request: impl tonic::IntoRequest<super::HoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HoldValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/HoldValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "HoldValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn release_hold_value(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseHoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReleaseHoldValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ReleaseHoldValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "ReleaseHoldValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn obtain_hold_value(
            &mut self,
            request: impl tonic::IntoRequest<super::ObtainHoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ObtainHoldValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ObtainHoldValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "ObtainHoldValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn value_advice(
            &mut self,
            request: impl tonic::IntoRequest<super::ValueAdviceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValueAdviceResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ValueAdvice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "ValueAdvice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::IssuerFromMicaServicePingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerFromMicaServicePingResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaService",
                        "Ping",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_from_mica_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerFromMicaServiceServer.
    #[async_trait]
    pub trait IssuerFromMicaService: std::marker::Send + std::marker::Sync + 'static {
        async fn obtain_value(
            &self,
            request: tonic::Request<super::ObtainValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ObtainValueResponse>,
            tonic::Status,
        >;
        async fn reverse_obtain_value(
            &self,
            request: tonic::Request<super::ReverseObtainValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseObtainValueResponse>,
            tonic::Status,
        >;
        async fn refund_value(
            &self,
            request: tonic::Request<super::RefundValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefundValueResponse>,
            tonic::Status,
        >;
        async fn payout_value(
            &self,
            request: tonic::Request<super::PayoutValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PayoutValueResponse>,
            tonic::Status,
        >;
        async fn hold_value(
            &self,
            request: tonic::Request<super::HoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HoldValueResponse>,
            tonic::Status,
        >;
        async fn release_hold_value(
            &self,
            request: tonic::Request<super::ReleaseHoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReleaseHoldValueResponse>,
            tonic::Status,
        >;
        async fn obtain_hold_value(
            &self,
            request: tonic::Request<super::ObtainHoldValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ObtainHoldValueResponse>,
            tonic::Status,
        >;
        async fn value_advice(
            &self,
            request: tonic::Request<super::ValueAdviceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValueAdviceResponse>,
            tonic::Status,
        >;
        async fn ping(
            &self,
            request: tonic::Request<super::IssuerFromMicaServicePingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerFromMicaServicePingResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IssuerFromMicaServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IssuerFromMicaServiceServer<T> {
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
    for IssuerFromMicaServiceServer<T>
    where
        T: IssuerFromMicaService,
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ObtainValue" => {
                    #[allow(non_camel_case_types)]
                    struct ObtainValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::ObtainValueRequest>
                    for ObtainValueSvc<T> {
                        type Response = super::ObtainValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ObtainValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::obtain_value(&inner, request)
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
                        let method = ObtainValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ReverseObtainValue" => {
                    #[allow(non_camel_case_types)]
                    struct ReverseObtainValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::ReverseObtainValueRequest>
                    for ReverseObtainValueSvc<T> {
                        type Response = super::ReverseObtainValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReverseObtainValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::reverse_obtain_value(
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
                        let method = ReverseObtainValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/RefundValue" => {
                    #[allow(non_camel_case_types)]
                    struct RefundValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::RefundValueRequest>
                    for RefundValueSvc<T> {
                        type Response = super::RefundValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RefundValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::refund_value(&inner, request)
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
                        let method = RefundValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/PayoutValue" => {
                    #[allow(non_camel_case_types)]
                    struct PayoutValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::PayoutValueRequest>
                    for PayoutValueSvc<T> {
                        type Response = super::PayoutValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PayoutValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::payout_value(&inner, request)
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
                        let method = PayoutValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/HoldValue" => {
                    #[allow(non_camel_case_types)]
                    struct HoldValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::HoldValueRequest>
                    for HoldValueSvc<T> {
                        type Response = super::HoldValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HoldValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::hold_value(&inner, request)
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
                        let method = HoldValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ReleaseHoldValue" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseHoldValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::ReleaseHoldValueRequest>
                    for ReleaseHoldValueSvc<T> {
                        type Response = super::ReleaseHoldValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseHoldValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::release_hold_value(
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
                        let method = ReleaseHoldValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ObtainHoldValue" => {
                    #[allow(non_camel_case_types)]
                    struct ObtainHoldValueSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::ObtainHoldValueRequest>
                    for ObtainHoldValueSvc<T> {
                        type Response = super::ObtainHoldValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ObtainHoldValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::obtain_hold_value(
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
                        let method = ObtainHoldValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/ValueAdvice" => {
                    #[allow(non_camel_case_types)]
                    struct ValueAdviceSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<super::ValueAdviceRequest>
                    for ValueAdviceSvc<T> {
                        type Response = super::ValueAdviceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValueAdviceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::value_advice(&inner, request)
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
                        let method = ValueAdviceSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaService/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: IssuerFromMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaService,
                    > tonic::server::UnaryService<
                        super::IssuerFromMicaServicePingRequest,
                    > for PingSvc<T> {
                        type Response = super::IssuerFromMicaServicePingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::IssuerFromMicaServicePingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaService>::ping(&inner, request).await
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
                        let method = PingSvc(inner);
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
    impl<T> Clone for IssuerFromMicaServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "mica.issuer.service.v2.IssuerFromMicaService";
    impl<T> tonic::server::NamedService for IssuerFromMicaServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod issuer_from_mica_wip_service_client {
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
    pub struct IssuerFromMicaWipServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerFromMicaWipServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerFromMicaWipServiceClient<T>
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
        ) -> IssuerFromMicaWipServiceClient<InterceptedService<T, F>>
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
            IssuerFromMicaWipServiceClient::new(
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
        pub async fn approve_chargeback(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveChargebackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApproveChargebackResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaWIPService/ApproveChargeback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaWIPService",
                        "ApproveChargeback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::value::v2::ReceiveValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::value::v2::ReceiveValueResponse>,
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
                "/mica.issuer.service.v2.IssuerFromMicaWIPService/ReceiveValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerFromMicaWIPService",
                        "ReceiveValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_from_mica_wip_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerFromMicaWipServiceServer.
    #[async_trait]
    pub trait IssuerFromMicaWipService: std::marker::Send + std::marker::Sync + 'static {
        async fn approve_chargeback(
            &self,
            request: tonic::Request<super::ApproveChargebackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApproveChargebackResponse>,
            tonic::Status,
        >;
        async fn receive_value(
            &self,
            request: tonic::Request<super::super::super::value::v2::ReceiveValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::value::v2::ReceiveValueResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IssuerFromMicaWipServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IssuerFromMicaWipServiceServer<T> {
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
    for IssuerFromMicaWipServiceServer<T>
    where
        T: IssuerFromMicaWipService,
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
                "/mica.issuer.service.v2.IssuerFromMicaWIPService/ApproveChargeback" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveChargebackSvc<T: IssuerFromMicaWipService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaWipService,
                    > tonic::server::UnaryService<super::ApproveChargebackRequest>
                    for ApproveChargebackSvc<T> {
                        type Response = super::ApproveChargebackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApproveChargebackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaWipService>::approve_chargeback(
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
                        let method = ApproveChargebackSvc(inner);
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
                "/mica.issuer.service.v2.IssuerFromMicaWIPService/ReceiveValue" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveValueSvc<T: IssuerFromMicaWipService>(pub Arc<T>);
                    impl<
                        T: IssuerFromMicaWipService,
                    > tonic::server::UnaryService<
                        super::super::super::value::v2::ReceiveValueRequest,
                    > for ReceiveValueSvc<T> {
                        type Response = super::super::super::value::v2::ReceiveValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::value::v2::ReceiveValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerFromMicaWipService>::receive_value(
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
                        let method = ReceiveValueSvc(inner);
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
    impl<T> Clone for IssuerFromMicaWipServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "mica.issuer.service.v2.IssuerFromMicaWIPService";
    impl<T> tonic::server::NamedService for IssuerFromMicaWipServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod issuer_to_mica_service_client {
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
    pub struct IssuerToMicaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerToMicaServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerToMicaServiceClient<T>
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
        ) -> IssuerToMicaServiceClient<InterceptedService<T, F>>
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
            IssuerToMicaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_issuer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::issuer::v2::GetIssuerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::issuer::v2::GetIssuerResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetIssuer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "GetIssuer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::user::v2::RegisterUserRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::RegisterUserResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/RegisterUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "RegisterUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns a registered user, looked up by Mica's user key or the Issuer's user reference.
*/
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::user::v2::GetUserRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::GetUserResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "GetUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Updates the demographic data Mica holds for a user. Pass the `version` last returned for the user; a stale version
 returns STATUS_VERSION_MISMATCH.
*/
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::user::v2::UpdateUserRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::UpdateUserResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/UpdateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "UpdateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Removes a user from Mica, typically when the user ends their relationship with the Issuer (for example, closes
 their account). Once removed, the user's `issuer_user_ref` can be registered again.
*/
        pub async fn remove_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::user::v2::RemoveUserRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::RemoveUserResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "RemoveUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Registers one of a user's instruments (accounts) with Mica and returns Mica's key for the instrument. The user must
 already be registered. Returns STATUS_DUPLICATE if the user already has an instrument with this
 `issuer_instrument_ref`.
*/
        pub async fn register_instrument(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::RegisterInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RegisterInstrumentResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/RegisterInstrument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "RegisterInstrument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns a registered instrument, looked up by Mica's instrument key or by the Issuer's user and instrument
 references.
*/
        pub async fn get_instrument(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::GetInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::instrument::v2::GetInstrumentResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetInstrument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "GetInstrument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Removes an instrument from Mica. Once removed, its `issuer_instrument_ref` can be registered again.
*/
        pub async fn remove_instrument(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::RemoveInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RemoveInstrumentResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveInstrument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "RemoveInstrument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns all instruments registered for a user.
*/
        pub async fn search_instrument(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::SearchInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::SearchInstrumentResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/SearchInstrument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "SearchInstrument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Creates a UUEK for one of the Issuer's instruments. The Issuer gives the UUEK to its user, who presents it to an
 Acceptor to pay from that instrument. The request controls how long, how often and for how much the UUEK can be
 used.
*/
        pub async fn provision_issuer_uuek(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::uuek::v2::ProvisionIssuerUuekRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::uuek::v2::ProvisionIssuerUuekResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/ProvisionIssuerUUEK",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "ProvisionIssuerUUEK",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Starts linking one of a user's instruments to another member by returning a linking code. The user enters the code
 at the other member (for example, a merchant) to save the instrument as a persistent payment method there.
*/
        pub async fn provision_instrument_linking_code(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/ProvisionInstrumentLinkingCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "ProvisionInstrumentLinkingCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Completes a link that another member started: the user brings a linking code from the other member, and the
 Issuer submits it here together with the user and instrument the link should attach to.
*/
        pub async fn complete_linking_with_code(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::CompleteLinkingWithCodeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::CompleteLinkingWithCodeResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/CompleteLinkingWithCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "CompleteLinkingWithCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns one instrument link by its `link_key`.
*/
        pub async fn get_instrument_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::GetInstrumentLinkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::GetInstrumentLinkResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetInstrumentLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "GetInstrumentLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns the established instrument links for a user, an instrument, or both.
*/
        pub async fn search_instrument_links(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::SearchInstrumentLinksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::SearchInstrumentLinksResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/SearchInstrumentLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "SearchInstrumentLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Removes an instrument link. Mica also notifies the member on the other side of the link so it is removed there.
*/
        pub async fn remove_instrument_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::instrument::v2::RemoveInstrumentLinkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RemoveInstrumentLinkResponse,
            >,
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveInstrumentLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "RemoveInstrumentLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Returns the records of a past transaction by Mica's global transaction id or the Issuer's transaction reference.
 Intended for pre-production testing: Mica purges transactions from its operational store shortly after they
 complete, and this call only reads that store. `oldest_transaction_time` in the response says how far back data is
 still available.
*/
        pub async fn get_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::value::v2::GetValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::value::v2::GetValueResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaService",
                        "GetValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_to_mica_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerToMicaServiceServer.
    #[async_trait]
    pub trait IssuerToMicaService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_issuer(
            &self,
            request: tonic::Request<super::super::super::issuer::v2::GetIssuerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::issuer::v2::GetIssuerResponse>,
            tonic::Status,
        >;
        async fn register_user(
            &self,
            request: tonic::Request<super::super::super::user::v2::RegisterUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::RegisterUserResponse>,
            tonic::Status,
        >;
        /** Returns a registered user, looked up by Mica's user key or the Issuer's user reference.
*/
        async fn get_user(
            &self,
            request: tonic::Request<super::super::super::user::v2::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::GetUserResponse>,
            tonic::Status,
        >;
        /** Updates the demographic data Mica holds for a user. Pass the `version` last returned for the user; a stale version
 returns STATUS_VERSION_MISMATCH.
*/
        async fn update_user(
            &self,
            request: tonic::Request<super::super::super::user::v2::UpdateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::UpdateUserResponse>,
            tonic::Status,
        >;
        /** Removes a user from Mica, typically when the user ends their relationship with the Issuer (for example, closes
 their account). Once removed, the user's `issuer_user_ref` can be registered again.
*/
        async fn remove_user(
            &self,
            request: tonic::Request<super::super::super::user::v2::RemoveUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::user::v2::RemoveUserResponse>,
            tonic::Status,
        >;
        /** Registers one of a user's instruments (accounts) with Mica and returns Mica's key for the instrument. The user must
 already be registered. Returns STATUS_DUPLICATE if the user already has an instrument with this
 `issuer_instrument_ref`.
*/
        async fn register_instrument(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::RegisterInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RegisterInstrumentResponse,
            >,
            tonic::Status,
        >;
        /** Returns a registered instrument, looked up by Mica's instrument key or by the Issuer's user and instrument
 references.
*/
        async fn get_instrument(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::GetInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::instrument::v2::GetInstrumentResponse>,
            tonic::Status,
        >;
        /** Removes an instrument from Mica. Once removed, its `issuer_instrument_ref` can be registered again.
*/
        async fn remove_instrument(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::RemoveInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RemoveInstrumentResponse,
            >,
            tonic::Status,
        >;
        /** Returns all instruments registered for a user.
*/
        async fn search_instrument(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::SearchInstrumentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::SearchInstrumentResponse,
            >,
            tonic::Status,
        >;
        /** Creates a UUEK for one of the Issuer's instruments. The Issuer gives the UUEK to its user, who presents it to an
 Acceptor to pay from that instrument. The request controls how long, how often and for how much the UUEK can be
 used.
*/
        async fn provision_issuer_uuek(
            &self,
            request: tonic::Request<
                super::super::super::uuek::v2::ProvisionIssuerUuekRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::uuek::v2::ProvisionIssuerUuekResponse>,
            tonic::Status,
        >;
        /** Starts linking one of a user's instruments to another member by returning a linking code. The user enters the code
 at the other member (for example, a merchant) to save the instrument as a persistent payment method there.
*/
        async fn provision_instrument_linking_code(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeResponse,
            >,
            tonic::Status,
        >;
        /** Completes a link that another member started: the user brings a linking code from the other member, and the
 Issuer submits it here together with the user and instrument the link should attach to.
*/
        async fn complete_linking_with_code(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::CompleteLinkingWithCodeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::CompleteLinkingWithCodeResponse,
            >,
            tonic::Status,
        >;
        /** Returns one instrument link by its `link_key`.
*/
        async fn get_instrument_link(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::GetInstrumentLinkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::GetInstrumentLinkResponse,
            >,
            tonic::Status,
        >;
        /** Returns the established instrument links for a user, an instrument, or both.
*/
        async fn search_instrument_links(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::SearchInstrumentLinksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::SearchInstrumentLinksResponse,
            >,
            tonic::Status,
        >;
        /** Removes an instrument link. Mica also notifies the member on the other side of the link so it is removed there.
*/
        async fn remove_instrument_link(
            &self,
            request: tonic::Request<
                super::super::super::instrument::v2::RemoveInstrumentLinkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::instrument::v2::RemoveInstrumentLinkResponse,
            >,
            tonic::Status,
        >;
        /** Returns the records of a past transaction by Mica's global transaction id or the Issuer's transaction reference.
 Intended for pre-production testing: Mica purges transactions from its operational store shortly after they
 complete, and this call only reads that store. `oldest_transaction_time` in the response says how far back data is
 still available.
*/
        async fn get_value(
            &self,
            request: tonic::Request<super::super::super::value::v2::GetValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::value::v2::GetValueResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IssuerToMicaServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IssuerToMicaServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IssuerToMicaServiceServer<T>
    where
        T: IssuerToMicaService,
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetIssuer" => {
                    #[allow(non_camel_case_types)]
                    struct GetIssuerSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::issuer::v2::GetIssuerRequest,
                    > for GetIssuerSvc<T> {
                        type Response = super::super::super::issuer::v2::GetIssuerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::issuer::v2::GetIssuerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::get_issuer(&inner, request)
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
                        let method = GetIssuerSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/RegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterUserSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::user::v2::RegisterUserRequest,
                    > for RegisterUserSvc<T> {
                        type Response = super::super::super::user::v2::RegisterUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::user::v2::RegisterUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::register_user(&inner, request)
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
                        let method = RegisterUserSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::user::v2::GetUserRequest,
                    > for GetUserSvc<T> {
                        type Response = super::super::super::user::v2::GetUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::user::v2::GetUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::get_user(&inner, request).await
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
                        let method = GetUserSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/UpdateUser" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::user::v2::UpdateUserRequest,
                    > for UpdateUserSvc<T> {
                        type Response = super::super::super::user::v2::UpdateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::user::v2::UpdateUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::update_user(&inner, request)
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
                        let method = UpdateUserSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveUser" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUserSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::user::v2::RemoveUserRequest,
                    > for RemoveUserSvc<T> {
                        type Response = super::super::super::user::v2::RemoveUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::user::v2::RemoveUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::remove_user(&inner, request)
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
                        let method = RemoveUserSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/RegisterInstrument" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterInstrumentSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::RegisterInstrumentRequest,
                    > for RegisterInstrumentSvc<T> {
                        type Response = super::super::super::instrument::v2::RegisterInstrumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::RegisterInstrumentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::register_instrument(
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
                        let method = RegisterInstrumentSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetInstrument" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstrumentSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::GetInstrumentRequest,
                    > for GetInstrumentSvc<T> {
                        type Response = super::super::super::instrument::v2::GetInstrumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::GetInstrumentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::get_instrument(&inner, request)
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
                        let method = GetInstrumentSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveInstrument" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveInstrumentSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::RemoveInstrumentRequest,
                    > for RemoveInstrumentSvc<T> {
                        type Response = super::super::super::instrument::v2::RemoveInstrumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::RemoveInstrumentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::remove_instrument(
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
                        let method = RemoveInstrumentSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/SearchInstrument" => {
                    #[allow(non_camel_case_types)]
                    struct SearchInstrumentSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::SearchInstrumentRequest,
                    > for SearchInstrumentSvc<T> {
                        type Response = super::super::super::instrument::v2::SearchInstrumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::SearchInstrumentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::search_instrument(
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
                        let method = SearchInstrumentSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/ProvisionIssuerUUEK" => {
                    #[allow(non_camel_case_types)]
                    struct ProvisionIssuerUUEKSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::uuek::v2::ProvisionIssuerUuekRequest,
                    > for ProvisionIssuerUUEKSvc<T> {
                        type Response = super::super::super::uuek::v2::ProvisionIssuerUuekResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::uuek::v2::ProvisionIssuerUuekRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::provision_issuer_uuek(
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
                        let method = ProvisionIssuerUUEKSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/ProvisionInstrumentLinkingCode" => {
                    #[allow(non_camel_case_types)]
                    struct ProvisionInstrumentLinkingCodeSvc<T: IssuerToMicaService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeRequest,
                    > for ProvisionInstrumentLinkingCodeSvc<T> {
                        type Response = super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::ProvisionInstrumentLinkingCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::provision_instrument_linking_code(
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
                        let method = ProvisionInstrumentLinkingCodeSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/CompleteLinkingWithCode" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteLinkingWithCodeSvc<T: IssuerToMicaService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::CompleteLinkingWithCodeRequest,
                    > for CompleteLinkingWithCodeSvc<T> {
                        type Response = super::super::super::instrument::v2::CompleteLinkingWithCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::CompleteLinkingWithCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::complete_linking_with_code(
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
                        let method = CompleteLinkingWithCodeSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetInstrumentLink" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstrumentLinkSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::GetInstrumentLinkRequest,
                    > for GetInstrumentLinkSvc<T> {
                        type Response = super::super::super::instrument::v2::GetInstrumentLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::GetInstrumentLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::get_instrument_link(
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
                        let method = GetInstrumentLinkSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/SearchInstrumentLinks" => {
                    #[allow(non_camel_case_types)]
                    struct SearchInstrumentLinksSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::SearchInstrumentLinksRequest,
                    > for SearchInstrumentLinksSvc<T> {
                        type Response = super::super::super::instrument::v2::SearchInstrumentLinksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::SearchInstrumentLinksRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::search_instrument_links(
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
                        let method = SearchInstrumentLinksSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/RemoveInstrumentLink" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveInstrumentLinkSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::instrument::v2::RemoveInstrumentLinkRequest,
                    > for RemoveInstrumentLinkSvc<T> {
                        type Response = super::super::super::instrument::v2::RemoveInstrumentLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::instrument::v2::RemoveInstrumentLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::remove_instrument_link(
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
                        let method = RemoveInstrumentLinkSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaService/GetValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetValueSvc<T: IssuerToMicaService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaService,
                    > tonic::server::UnaryService<
                        super::super::super::value::v2::GetValueRequest,
                    > for GetValueSvc<T> {
                        type Response = super::super::super::value::v2::GetValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::value::v2::GetValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaService>::get_value(&inner, request).await
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
                        let method = GetValueSvc(inner);
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
    impl<T> Clone for IssuerToMicaServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "mica.issuer.service.v2.IssuerToMicaService";
    impl<T> tonic::server::NamedService for IssuerToMicaServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod issuer_to_mica_wip_service_client {
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
    pub struct IssuerToMicaWipServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerToMicaWipServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerToMicaWipServiceClient<T>
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
        ) -> IssuerToMicaWipServiceClient<InterceptedService<T, F>>
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
            IssuerToMicaWipServiceClient::new(
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
        pub async fn search_recurring_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRecurringPaymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRecurringPaymentsResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/SearchRecurringPayments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaWIPService",
                        "SearchRecurringPayments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_value(
            &mut self,
            request: impl tonic::IntoRequest<super::SendValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendValueResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/SendValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaWIPService",
                        "SendValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn initialize_web_session(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeWebSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitializeWebSessionResponse>,
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/InitializeWebSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mica.issuer.service.v2.IssuerToMicaWIPService",
                        "InitializeWebSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_to_mica_wip_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerToMicaWipServiceServer.
    #[async_trait]
    pub trait IssuerToMicaWipService: std::marker::Send + std::marker::Sync + 'static {
        async fn search_recurring_payments(
            &self,
            request: tonic::Request<super::SearchRecurringPaymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRecurringPaymentsResponse>,
            tonic::Status,
        >;
        async fn send_value(
            &self,
            request: tonic::Request<super::SendValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendValueResponse>,
            tonic::Status,
        >;
        async fn initialize_web_session(
            &self,
            request: tonic::Request<super::InitializeWebSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitializeWebSessionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IssuerToMicaWipServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IssuerToMicaWipServiceServer<T> {
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
    for IssuerToMicaWipServiceServer<T>
    where
        T: IssuerToMicaWipService,
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/SearchRecurringPayments" => {
                    #[allow(non_camel_case_types)]
                    struct SearchRecurringPaymentsSvc<T: IssuerToMicaWipService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerToMicaWipService,
                    > tonic::server::UnaryService<super::SearchRecurringPaymentsRequest>
                    for SearchRecurringPaymentsSvc<T> {
                        type Response = super::SearchRecurringPaymentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SearchRecurringPaymentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaWipService>::search_recurring_payments(
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
                        let method = SearchRecurringPaymentsSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/SendValue" => {
                    #[allow(non_camel_case_types)]
                    struct SendValueSvc<T: IssuerToMicaWipService>(pub Arc<T>);
                    impl<
                        T: IssuerToMicaWipService,
                    > tonic::server::UnaryService<super::SendValueRequest>
                    for SendValueSvc<T> {
                        type Response = super::SendValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaWipService>::send_value(&inner, request)
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
                        let method = SendValueSvc(inner);
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
                "/mica.issuer.service.v2.IssuerToMicaWIPService/InitializeWebSession" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeWebSessionSvc<T: IssuerToMicaWipService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: IssuerToMicaWipService,
                    > tonic::server::UnaryService<super::InitializeWebSessionRequest>
                    for InitializeWebSessionSvc<T> {
                        type Response = super::InitializeWebSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeWebSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IssuerToMicaWipService>::initialize_web_session(
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
                        let method = InitializeWebSessionSvc(inner);
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
    impl<T> Clone for IssuerToMicaWipServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "mica.issuer.service.v2.IssuerToMicaWIPService";
    impl<T> tonic::server::NamedService for IssuerToMicaWipServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
