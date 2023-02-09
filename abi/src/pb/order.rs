#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchasedProduct {
    #[prost(string, tag = "1")]
    pub product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub product_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub quantity: u32,
    #[prost(uint32, tag = "4")]
    pub unit_price: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub cart: ::prost::alloc::vec::Vec<PurchasedProduct>,
    #[prost(enumeration = "OrderStatus", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub create_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub update_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub note: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderAddRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub cart: ::prost::alloc::vec::Vec<PurchasedProduct>,
    #[prost(enumeration = "OrderStatus", tag = "3")]
    pub status: i32,
    #[prost(message, optional, tag = "4")]
    pub create_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub note: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderAddResponse {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCancelRequest {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCancelResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderModifyRequest {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub cart: ::prost::alloc::vec::Vec<PurchasedProduct>,
    #[prost(enumeration = "OrderStatus", tag = "3")]
    pub status: i32,
    #[prost(message, optional, tag = "4")]
    pub update_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub note: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderModifyResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// if field is empty, then query all
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderQueryRequest {
    #[prost(string, optional, tag = "1")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// is status is unknown, then query all
    #[prost(enumeration = "OrderStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub create_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub update_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderListenRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderListenResponse {
    #[prost(enumeration = "OrderUpdateType", tag = "1")]
    pub op: i32,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<Order>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    Unknown = 0,
    PaymentPending = 1,
    Paid = 2,
    Canceled = 3,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unknown => "ORDER_STATUS_UNKNOWN",
            OrderStatus::PaymentPending => "ORDER_STATUS_PAYMENT_PENDING",
            OrderStatus::Paid => "ORDER_STATUS_PAID",
            OrderStatus::Canceled => "ORDER_STATUS_CANCELED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_STATUS_UNKNOWN" => Some(Self::Unknown),
            "ORDER_STATUS_PAYMENT_PENDING" => Some(Self::PaymentPending),
            "ORDER_STATUS_PAID" => Some(Self::Paid),
            "ORDER_STATUS_CANCELED" => Some(Self::Canceled),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderUpdateType {
    Unknown = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
}
impl OrderUpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderUpdateType::Unknown => "ORDER_UPDATE_TYPE_UNKNOWN",
            OrderUpdateType::Create => "ORDER_UPDATE_TYPE_CREATE",
            OrderUpdateType::Update => "ORDER_UPDATE_TYPE_UPDATE",
            OrderUpdateType::Delete => "ORDER_UPDATE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_UPDATE_TYPE_UNKNOWN" => Some(Self::Unknown),
            "ORDER_UPDATE_TYPE_CREATE" => Some(Self::Create),
            "ORDER_UPDATE_TYPE_UPDATE" => Some(Self::Update),
            "ORDER_UPDATE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrderServiceClient<T>
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
        ) -> OrderServiceClient<InterceptedService<T, F>>
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
            OrderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn order_add(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderAddRequest>,
        ) -> Result<tonic::Response<super::OrderAddResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order.OrderService/orderAdd");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn order_cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderCancelRequest>,
        ) -> Result<tonic::Response<super::OrderCancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order.OrderService/orderCancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn order_modify(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderModifyRequest>,
        ) -> Result<tonic::Response<super::OrderModifyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order.OrderService/orderModify");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn order_list(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderQueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Order>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order.OrderService/orderList");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// another system could monitor newly added/confirmed/cancelled orders
        pub async fn listen(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderListenRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Order>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order.OrderService/listen");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod order_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrderServiceServer.
    #[async_trait]
    pub trait OrderService: Send + Sync + 'static {
        async fn order_add(
            &self,
            request: tonic::Request<super::OrderAddRequest>,
        ) -> Result<tonic::Response<super::OrderAddResponse>, tonic::Status>;
        async fn order_cancel(
            &self,
            request: tonic::Request<super::OrderCancelRequest>,
        ) -> Result<tonic::Response<super::OrderCancelResponse>, tonic::Status>;
        async fn order_modify(
            &self,
            request: tonic::Request<super::OrderModifyRequest>,
        ) -> Result<tonic::Response<super::OrderModifyResponse>, tonic::Status>;
        /// Server streaming response type for the orderList method.
        type orderListStream: futures_core::Stream<Item = Result<super::Order, tonic::Status>>
            + Send
            + 'static;
        async fn order_list(
            &self,
            request: tonic::Request<super::OrderQueryRequest>,
        ) -> Result<tonic::Response<Self::orderListStream>, tonic::Status>;
        /// Server streaming response type for the listen method.
        type listenStream: futures_core::Stream<Item = Result<super::Order, tonic::Status>>
            + Send
            + 'static;
        /// another system could monitor newly added/confirmed/cancelled orders
        async fn listen(
            &self,
            request: tonic::Request<super::OrderListenRequest>,
        ) -> Result<tonic::Response<Self::listenStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OrderServiceServer<T: OrderService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OrderService> OrderServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderServiceServer<T>
    where
        T: OrderService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/order.OrderService/orderAdd" => {
                    #[allow(non_camel_case_types)]
                    struct orderAddSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::OrderAddRequest> for orderAddSvc<T> {
                        type Response = super::OrderAddResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderAddRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order_add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = orderAddSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/order.OrderService/orderCancel" => {
                    #[allow(non_camel_case_types)]
                    struct orderCancelSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::OrderCancelRequest> for orderCancelSvc<T> {
                        type Response = super::OrderCancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderCancelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order_cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = orderCancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/order.OrderService/orderModify" => {
                    #[allow(non_camel_case_types)]
                    struct orderModifySvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::OrderModifyRequest> for orderModifySvc<T> {
                        type Response = super::OrderModifyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderModifyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order_modify(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = orderModifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/order.OrderService/orderList" => {
                    #[allow(non_camel_case_types)]
                    struct orderListSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService>
                        tonic::server::ServerStreamingService<super::OrderQueryRequest>
                        for orderListSvc<T>
                    {
                        type Response = super::Order;
                        type ResponseStream = T::orderListStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = orderListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/order.OrderService/listen" => {
                    #[allow(non_camel_case_types)]
                    struct listenSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService>
                        tonic::server::ServerStreamingService<super::OrderListenRequest>
                        for listenSvc<T>
                    {
                        type Response = super::Order;
                        type ResponseStream = T::listenStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderListenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).listen(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: OrderService> Clone for OrderServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: OrderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderService> tonic::server::NamedService for OrderServiceServer<T> {
        const NAME: &'static str = "order.OrderService";
    }
}
