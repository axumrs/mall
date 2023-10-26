#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct U32Range {
    #[prost(uint32, tag = "1")]
    pub min: u32,
    #[prost(uint32, tag = "2")]
    pub max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct U64Range {
    #[prost(uint64, tag = "1")]
    pub min: u64,
    #[prost(uint64, tag = "2")]
    pub max: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(enumeration = "UserStatus", tag = "5")]
    pub status: i32,
    #[prost(message, optional, tag = "6")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "7")]
    pub is_del: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserStatus {
    Pending = 0,
    Actived = 1,
    Freezed = 2,
}
impl UserStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserStatus::Pending => "PENDING",
            UserStatus::Actived => "ACTIVED",
            UserStatus::Freezed => "FREEZED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PENDING" => Some(Self::Pending),
            "ACTIVED" => Some(Self::Actived),
            "FREEZED" => Some(Self::Freezed),
            _ => None,
        }
    }
}
/// 地址详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDetail {
    /// 收件人
    #[prost(string, tag = "1")]
    pub consignee: ::prost::alloc::string::String,
    /// 电话
    #[prost(string, tag = "2")]
    pub phone: ::prost::alloc::string::String,
    /// 详细地址
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// 省市区【省级】
    #[prost(string, tag = "4")]
    pub province: ::prost::alloc::string::String,
    /// 城市【地级】
    #[prost(string, tag = "5")]
    pub city: ::prost::alloc::string::String,
    /// 县市区【县级】
    #[prost(string, tag = "6")]
    pub county: ::prost::alloc::string::String,
    /// 邮编
    #[prost(string, tag = "7")]
    pub post_code: ::prost::alloc::string::String,
}
/// 地址
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 是否默认地址
    #[prost(bool, tag = "3")]
    pub is_default: bool,
    /// 是否删除
    #[prost(bool, tag = "4")]
    pub is_del: bool,
    /// 添加时间
    #[prost(message, optional, tag = "5")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    /// 地址详情
    #[prost(message, optional, tag = "6")]
    pub detail: ::core::option::Option<AddressDetail>,
}
/// 订单
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 购物车ID
    #[prost(string, tag = "3")]
    pub cart_id: ::prost::alloc::string::String,
    /// 编号
    #[prost(string, tag = "4")]
    pub sn: ::prost::alloc::string::String,
    /// 状态
    #[prost(enumeration = "OrderStatus", tag = "5")]
    pub status: i32,
    /// 金额
    #[prost(uint32, tag = "6")]
    pub amount: u32,
    /// 总数
    #[prost(uint32, tag = "7")]
    pub total_num: u32,
    /// 运费
    #[prost(uint32, tag = "8")]
    pub freight: u32,
    /// 收货地址
    #[prost(message, optional, tag = "9")]
    pub address: ::core::option::Option<AddressDetail>,
    /// 快递单号
    #[prost(string, tag = "10")]
    pub delivery_id: ::prost::alloc::string::String,
    /// 下单时间
    #[prost(message, optional, tag = "11")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    /// 自动取消订单的时间
    #[prost(message, optional, tag = "12")]
    pub cancel_until_dateline: ::core::option::Option<::prost_types::Timestamp>,
    /// 自动确认订单的时间
    #[prost(message, optional, tag = "13")]
    pub confirm_until_dateline: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "14")]
    pub is_del: bool,
}
/// 订单状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    /// 未知
    Unspecified = 0,
    /// 待支付（已创建订单）
    PendingPay = 1,
    /// 用户取消
    UserCancel = 2,
    /// 管理员取消
    AdminCancel = 3,
    /// 超时自动取消
    TimeoutCancel = 4,
    /// 已支付（等待发货）
    Paied = 5,
    /// 运输中（已发货）
    Delivering = 6,
    /// 已送达（等待用户确认）
    Delivered = 7,
    /// 完成（用户确认收货）
    UserConfirmedDone = 8,
    /// 完成（自动确认收货）
    AutoConfirmedDone = 9,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unspecified => "Unspecified",
            OrderStatus::PendingPay => "PendingPay",
            OrderStatus::UserCancel => "UserCancel",
            OrderStatus::AdminCancel => "AdminCancel",
            OrderStatus::TimeoutCancel => "TimeoutCancel",
            OrderStatus::Paied => "Paied",
            OrderStatus::Delivering => "Delivering",
            OrderStatus::Delivered => "Delivered",
            OrderStatus::UserConfirmedDone => "UserConfirmedDone",
            OrderStatus::AutoConfirmedDone => "AutoConfirmedDone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "PendingPay" => Some(Self::PendingPay),
            "UserCancel" => Some(Self::UserCancel),
            "AdminCancel" => Some(Self::AdminCancel),
            "TimeoutCancel" => Some(Self::TimeoutCancel),
            "Paied" => Some(Self::Paied),
            "Delivering" => Some(Self::Delivering),
            "Delivered" => Some(Self::Delivered),
            "UserConfirmedDone" => Some(Self::UserConfirmedDone),
            "AutoConfirmedDone" => Some(Self::AutoConfirmedDone),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrRestoreRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_del: bool,
}
/// 购物车
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cart {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 商品ID
    #[prost(string, tag = "3")]
    pub goods_id: ::prost::alloc::string::String,
    /// 商品SKU
    #[prost(string, tag = "4")]
    pub goods_sku: ::prost::alloc::string::String,
    /// 购买数量
    #[prost(uint32, tag = "5")]
    pub num: u32,
    /// 金额
    #[prost(uint32, tag = "6")]
    pub amount: u32,
    /// 加入购物车时间
    #[prost(message, optional, tag = "7")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    /// hash
    #[prost(string, tag = "8")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aff {
    #[prost(uint64, tag = "1")]
    pub rows: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// 购物车详情响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CartDetailResponse {
    /// 购物车
    #[prost(message, repeated, tag = "1")]
    pub cart: ::prost::alloc::vec::Vec<Cart>,
}
/// 添加到购物车响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddItemToCartResponse {
    /// 购物车详情
    #[prost(message, optional, tag = "1")]
    pub detail: ::core::option::Option<CartDetailResponse>,
    /// 新增的ID
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<Id>,
}
/// 获取购物车
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCartRequest {
    /// 用户ID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
/// 清空购物车请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCartRequest {
    /// 用户ID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
/// 从购物车删除请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveItemFromCartRequest {
    /// 用户ID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 要删除购物车项的ID列表
    #[prost(string, repeated, tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 从购物车删除响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveItemFromCartResponse {
    /// 购物车详情
    #[prost(message, optional, tag = "1")]
    pub detail: ::core::option::Option<CartDetailResponse>,
    /// 已删除的项
    #[prost(message, optional, tag = "2")]
    pub removed: ::core::option::Option<CartDetailResponse>,
}
/// 购物车数量
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CartItemNum {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub num: u32,
}
/// 更新购物车某项的数量请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCartItemNumRequest {
    /// 购物车数量
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CartItemNum>,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
/// 更新购物车某项的数量响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCartItemNumResponse {
    /// 购物车详情
    #[prost(message, optional, tag = "1")]
    pub detail: ::core::option::Option<CartDetailResponse>,
    /// 收影响的行数
    #[prost(message, repeated, tag = "2")]
    pub affs: ::prost::alloc::vec::Vec<Aff>,
}
/// Generated client implementations.
pub mod cart_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 购物车服务
    #[derive(Debug, Clone)]
    pub struct CartServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CartServiceClient<tonic::transport::Channel> {
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
    impl<T> CartServiceClient<T>
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
        ) -> CartServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CartServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 添加到购物车
        pub async fn add_item_to_cart(
            &mut self,
            request: impl tonic::IntoRequest<super::Cart>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemToCartResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CartService/AddItemToCart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.CartService", "AddItemToCart"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取购物车
        pub async fn get_cart(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CartDetailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.CartService/GetCart");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.CartService", "GetCart"));
            self.inner.unary(req, path, codec).await
        }
        /// 清空购物车
        pub async fn clear_cart(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearCartRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.CartService/ClearCart");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.CartService", "ClearCart"));
            self.inner.unary(req, path, codec).await
        }
        /// 从购物车删除
        pub async fn remove_item_from_cart(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveItemFromCartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveItemFromCartResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CartService/RemoveItemFromCart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.CartService", "RemoveItemFromCart"));
            self.inner.unary(req, path, codec).await
        }
        /// 更新购物车某项的数量
        pub async fn update_cart_item_num(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCartItemNumRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCartItemNumResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CartService/UpdateCartItemNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.CartService", "UpdateCartItemNum"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cart_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CartServiceServer.
    #[async_trait]
    pub trait CartService: Send + Sync + 'static {
        /// 添加到购物车
        async fn add_item_to_cart(
            &self,
            request: tonic::Request<super::Cart>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemToCartResponse>,
            tonic::Status,
        >;
        /// 获取购物车
        async fn get_cart(
            &self,
            request: tonic::Request<super::GetCartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CartDetailResponse>,
            tonic::Status,
        >;
        /// 清空购物车
        async fn clear_cart(
            &self,
            request: tonic::Request<super::ClearCartRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 从购物车删除
        async fn remove_item_from_cart(
            &self,
            request: tonic::Request<super::RemoveItemFromCartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveItemFromCartResponse>,
            tonic::Status,
        >;
        /// 更新购物车某项的数量
        async fn update_cart_item_num(
            &self,
            request: tonic::Request<super::UpdateCartItemNumRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCartItemNumResponse>,
            tonic::Status,
        >;
    }
    /// 购物车服务
    #[derive(Debug)]
    pub struct CartServiceServer<T: CartService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CartService> CartServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CartServiceServer<T>
    where
        T: CartService,
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
                "/pb.CartService/AddItemToCart" => {
                    #[allow(non_camel_case_types)]
                    struct AddItemToCartSvc<T: CartService>(pub Arc<T>);
                    impl<T: CartService> tonic::server::UnaryService<super::Cart>
                    for AddItemToCartSvc<T> {
                        type Response = super::AddItemToCartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Cart>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_item_to_cart(request).await
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
                        let method = AddItemToCartSvc(inner);
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
                "/pb.CartService/GetCart" => {
                    #[allow(non_camel_case_types)]
                    struct GetCartSvc<T: CartService>(pub Arc<T>);
                    impl<
                        T: CartService,
                    > tonic::server::UnaryService<super::GetCartRequest>
                    for GetCartSvc<T> {
                        type Response = super::CartDetailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_cart(request).await };
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
                        let method = GetCartSvc(inner);
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
                "/pb.CartService/ClearCart" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCartSvc<T: CartService>(pub Arc<T>);
                    impl<
                        T: CartService,
                    > tonic::server::UnaryService<super::ClearCartRequest>
                    for ClearCartSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).clear_cart(request).await };
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
                        let method = ClearCartSvc(inner);
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
                "/pb.CartService/RemoveItemFromCart" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveItemFromCartSvc<T: CartService>(pub Arc<T>);
                    impl<
                        T: CartService,
                    > tonic::server::UnaryService<super::RemoveItemFromCartRequest>
                    for RemoveItemFromCartSvc<T> {
                        type Response = super::RemoveItemFromCartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveItemFromCartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_item_from_cart(request).await
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
                        let method = RemoveItemFromCartSvc(inner);
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
                "/pb.CartService/UpdateCartItemNum" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCartItemNumSvc<T: CartService>(pub Arc<T>);
                    impl<
                        T: CartService,
                    > tonic::server::UnaryService<super::UpdateCartItemNumRequest>
                    for UpdateCartItemNumSvc<T> {
                        type Response = super::UpdateCartItemNumResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCartItemNumRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_cart_item_num(request).await
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
                        let method = UpdateCartItemNumSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CartService> Clone for CartServiceServer<T> {
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
    impl<T: CartService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CartService> tonic::server::NamedService for CartServiceServer<T> {
        const NAME: &'static str = "pb.CartService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Goods {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub category_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sn: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_sale: bool,
    #[prost(uint32, tag = "6")]
    pub ship_fee: u32,
    #[prost(bool, tag = "7")]
    pub is_new: bool,
    #[prost(bool, tag = "8")]
    pub is_hot: bool,
    #[prost(uint64, tag = "9")]
    pub hit: u64,
    #[prost(uint64, tag = "10")]
    pub sold_num: u64,
    #[prost(uint64, tag = "11")]
    pub fav_num: u64,
    #[prost(uint32, tag = "12")]
    pub origin_price: u32,
    #[prost(uint32, tag = "13")]
    pub price: u32,
    #[prost(string, tag = "14")]
    pub brief: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub cover: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "16")]
    pub images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "17")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "18")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "19")]
    pub is_del: bool,
}
/// 分页
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paginate {
    /// 总记录数
    #[prost(uint32, tag = "1")]
    pub total: u32,
    /// 页码
    #[prost(uint32, tag = "2")]
    pub page: u32,
    /// 每页条数
    #[prost(uint32, tag = "3")]
    pub page_size: u32,
    /// 总页数
    #[prost(uint32, tag = "4")]
    pub total_page: u32,
}
/// 分页请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginateRequest {
    /// 页码
    #[prost(uint32, optional, tag = "1")]
    pub page: ::core::option::Option<u32>,
    /// 每页条数
    #[prost(uint32, optional, tag = "2")]
    pub page_size: ::core::option::Option<u32>,
}
/// 查找地址请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindAddressRequest {
    /// 用户ID。如果是用户进行操作，必须指定该参数
    #[prost(string, optional, tag = "3")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(oneof = "find_address_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_address_request::By>,
}
/// Nested message and enum types in `FindAddressRequest`.
pub mod find_address_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 通过ID查找
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// 查找默认地址
        #[prost(bool, tag = "2")]
        IsDefault(bool),
    }
}
/// 查找地址响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
}
/// 地址列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAddressRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    #[prost(string, optional, tag = "2")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub consignee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
    /// 详细地址
    #[prost(string, optional, tag = "5")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub is_del: ::core::option::Option<bool>,
}
/// 地址列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub address_list: ::prost::alloc::vec::Vec<Address>,
}
/// 删除/还原地址请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrRestoreAddressRequest {
    #[prost(message, optional, tag = "1")]
    pub dos: ::core::option::Option<DeleteOrRestoreRequest>,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    #[prost(string, optional, tag = "2")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// 获取默认地址请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultAddressRequest {
    /// 用户ID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
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
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
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
        /// 添加地址
        pub async fn create_address(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/CreateAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "CreateAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改地址
        pub async fn edit_address(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/EditAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "EditAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除/还原地址
        pub async fn delete_or_restore_address(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreAddressRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/DeleteOrRestoreAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "DeleteOrRestoreAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找地址
        pub async fn find_address(
            &mut self,
            request: impl tonic::IntoRequest<super::FindAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/FindAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "FindAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 地址列表
        pub async fn list_address(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/ListAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "ListAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 设置默认地址
        pub async fn set_default_address(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/SetDefaultAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "SetDefaultAddress"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取默认地址
        pub async fn get_default_address(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.OrderService/GetDefaultAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.OrderService", "GetDefaultAddress"));
            self.inner.unary(req, path, codec).await
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
        /// 添加地址
        async fn create_address(
            &self,
            request: tonic::Request<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        /// 修改地址
        async fn edit_address(
            &self,
            request: tonic::Request<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除/还原地址
        async fn delete_or_restore_address(
            &self,
            request: tonic::Request<super::DeleteOrRestoreAddressRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 查找地址
        async fn find_address(
            &self,
            request: tonic::Request<super::FindAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindAddressResponse>,
            tonic::Status,
        >;
        /// 地址列表
        async fn list_address(
            &self,
            request: tonic::Request<super::ListAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAddressResponse>,
            tonic::Status,
        >;
        /// 设置默认地址
        async fn set_default_address(
            &self,
            request: tonic::Request<super::Address>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 获取默认地址
        async fn get_default_address(
            &self,
            request: tonic::Request<super::GetDefaultAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindAddressResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OrderServiceServer<T: OrderService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderServiceServer<T>
    where
        T: OrderService,
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
                "/pb.OrderService/CreateAddress" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::Address>
                    for CreateAddressSvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_address(request).await
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
                        let method = CreateAddressSvc(inner);
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
                "/pb.OrderService/EditAddress" => {
                    #[allow(non_camel_case_types)]
                    struct EditAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::Address>
                    for EditAddressSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_address(request).await
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
                        let method = EditAddressSvc(inner);
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
                "/pb.OrderService/DeleteOrRestoreAddress" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreAddressRequest>
                    for DeleteOrRestoreAddressSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_address(request).await
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
                        let method = DeleteOrRestoreAddressSvc(inner);
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
                "/pb.OrderService/FindAddress" => {
                    #[allow(non_camel_case_types)]
                    struct FindAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::FindAddressRequest>
                    for FindAddressSvc<T> {
                        type Response = super::FindAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).find_address(request).await
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
                        let method = FindAddressSvc(inner);
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
                "/pb.OrderService/ListAddress" => {
                    #[allow(non_camel_case_types)]
                    struct ListAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::ListAddressRequest>
                    for ListAddressSvc<T> {
                        type Response = super::ListAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_address(request).await
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
                        let method = ListAddressSvc(inner);
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
                "/pb.OrderService/SetDefaultAddress" => {
                    #[allow(non_camel_case_types)]
                    struct SetDefaultAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::Address>
                    for SetDefaultAddressSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_default_address(request).await
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
                        let method = SetDefaultAddressSvc(inner);
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
                "/pb.OrderService/GetDefaultAddress" => {
                    #[allow(non_camel_case_types)]
                    struct GetDefaultAddressSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::GetDefaultAddressRequest>
                    for GetDefaultAddressSvc<T> {
                        type Response = super::FindAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDefaultAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_default_address(request).await
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
                        let method = GetDefaultAddressSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
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
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: OrderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderService> tonic::server::NamedService for OrderServiceServer<T> {
        const NAME: &'static str = "pb.OrderService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Banner {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub img: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub sort: i32,
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_del: bool,
}
/// 是否存在
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsExistsResponse {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub logo: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_del: bool,
    #[prost(message, optional, tag = "5")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Category {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration = "CategoryLevel", tag = "5")]
    pub level: i32,
    #[prost(message, optional, tag = "6")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "7")]
    pub is_del: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryTree {
    #[prost(message, optional, tag = "1")]
    pub category: ::core::option::Option<Category>,
    #[prost(string, tag = "2")]
    pub fullname: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CategoryLevel {
    Unspecified = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}
impl CategoryLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CategoryLevel::Unspecified => "UNSPECIFIED",
            CategoryLevel::Level1 => "LEVEL_1",
            CategoryLevel::Level2 => "LEVEL_2",
            CategoryLevel::Level3 => "LEVEL_3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "LEVEL_1" => Some(Self::Level1),
            "LEVEL_2" => Some(Self::Level2),
            "LEVEL_3" => Some(Self::Level3),
            _ => None,
        }
    }
}
/// 查找轮播图请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBannerRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
}
/// 查找轮播图响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBannerResponse {
    #[prost(message, optional, tag = "1")]
    pub banner: ::core::option::Option<Banner>,
}
/// 轮播图列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBannerRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(bool, tag = "4")]
    pub order_by_sort: bool,
}
/// 轮播图列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBannerResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub banners: ::prost::alloc::vec::Vec<Banner>,
}
/// 前N个轮播图请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopNBannerRequest {
    #[prost(int32, tag = "1")]
    pub num: i32,
    #[prost(bool, tag = "2")]
    pub order_by_id: bool,
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
}
/// 前N个轮播图响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopNBannerResponse {
    #[prost(message, repeated, tag = "1")]
    pub banners: ::prost::alloc::vec::Vec<Banner>,
}
/// Generated client implementations.
pub mod banner_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BannerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BannerServiceClient<tonic::transport::Channel> {
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
    impl<T> BannerServiceClient<T>
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
        ) -> BannerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BannerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建轮播图
        pub async fn create_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::Banner>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/CreateBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "CreateBanner"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改轮播图
        pub async fn edit_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::Banner>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/EditBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "EditBanner"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除/还原轮播图
        pub async fn delete_or_restore_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/DeleteOrRestoreBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "DeleteOrRestoreBanner"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找轮播图
        pub async fn find_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::FindBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBannerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/FindBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "FindBanner"));
            self.inner.unary(req, path, codec).await
        }
        /// 轮播图列表
        pub async fn list_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBannerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/ListBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "ListBanner"));
            self.inner.unary(req, path, codec).await
        }
        /// 前N个轮播图
        pub async fn top_n_banner(
            &mut self,
            request: impl tonic::IntoRequest<super::TopNBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TopNBannerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.BannerService/TopNBanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.BannerService", "TopNBanner"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod banner_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BannerServiceServer.
    #[async_trait]
    pub trait BannerService: Send + Sync + 'static {
        /// 创建轮播图
        async fn create_banner(
            &self,
            request: tonic::Request<super::Banner>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        /// 修改轮播图
        async fn edit_banner(
            &self,
            request: tonic::Request<super::Banner>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除/还原轮播图
        async fn delete_or_restore_banner(
            &self,
            request: tonic::Request<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 查找轮播图
        async fn find_banner(
            &self,
            request: tonic::Request<super::FindBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBannerResponse>,
            tonic::Status,
        >;
        /// 轮播图列表
        async fn list_banner(
            &self,
            request: tonic::Request<super::ListBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBannerResponse>,
            tonic::Status,
        >;
        /// 前N个轮播图
        async fn top_n_banner(
            &self,
            request: tonic::Request<super::TopNBannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TopNBannerResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BannerServiceServer<T: BannerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BannerService> BannerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BannerServiceServer<T>
    where
        T: BannerService,
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
                "/pb.BannerService/CreateBanner" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<T: BannerService> tonic::server::UnaryService<super::Banner>
                    for CreateBannerSvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Banner>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_banner(request).await
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
                        let method = CreateBannerSvc(inner);
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
                "/pb.BannerService/EditBanner" => {
                    #[allow(non_camel_case_types)]
                    struct EditBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<T: BannerService> tonic::server::UnaryService<super::Banner>
                    for EditBannerSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Banner>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).edit_banner(request).await };
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
                        let method = EditBannerSvc(inner);
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
                "/pb.BannerService/DeleteOrRestoreBanner" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<
                        T: BannerService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreRequest>
                    for DeleteOrRestoreBannerSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_banner(request).await
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
                        let method = DeleteOrRestoreBannerSvc(inner);
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
                "/pb.BannerService/FindBanner" => {
                    #[allow(non_camel_case_types)]
                    struct FindBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<
                        T: BannerService,
                    > tonic::server::UnaryService<super::FindBannerRequest>
                    for FindBannerSvc<T> {
                        type Response = super::FindBannerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindBannerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).find_banner(request).await };
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
                        let method = FindBannerSvc(inner);
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
                "/pb.BannerService/ListBanner" => {
                    #[allow(non_camel_case_types)]
                    struct ListBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<
                        T: BannerService,
                    > tonic::server::UnaryService<super::ListBannerRequest>
                    for ListBannerSvc<T> {
                        type Response = super::ListBannerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBannerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_banner(request).await };
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
                        let method = ListBannerSvc(inner);
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
                "/pb.BannerService/TopNBanner" => {
                    #[allow(non_camel_case_types)]
                    struct TopNBannerSvc<T: BannerService>(pub Arc<T>);
                    impl<
                        T: BannerService,
                    > tonic::server::UnaryService<super::TopNBannerRequest>
                    for TopNBannerSvc<T> {
                        type Response = super::TopNBannerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TopNBannerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).top_n_banner(request).await
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
                        let method = TopNBannerSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BannerService> Clone for BannerServiceServer<T> {
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
    impl<T: BannerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BannerService> tonic::server::NamedService for BannerServiceServer<T> {
        const NAME: &'static str = "pb.BannerService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryWithBrands {
    #[prost(message, optional, tag = "1")]
    pub category: ::core::option::Option<Category>,
    #[prost(message, repeated, tag = "2")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandWithCategoies {
    #[prost(message, optional, tag = "1")]
    pub brand: ::core::option::Option<Brand>,
    #[prost(message, repeated, tag = "2")]
    pub categoies: ::prost::alloc::vec::Vec<Category>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryWithBrandsTree {
    #[prost(message, optional, tag = "1")]
    pub category_with_brands: ::core::option::Option<CategoryWithBrands>,
    #[prost(string, tag = "2")]
    pub fullname: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsSku {
    #[prost(message, optional, tag = "1")]
    pub meta: ::core::option::Option<goods_sku::Meta>,
    #[prost(map = "string, message", tag = "2")]
    pub data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        goods_sku::DataItem,
    >,
}
/// Nested message and enum types in `GoodsSKU`.
pub mod goods_sku {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaItems {
        #[prost(string, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Meta {
        #[prost(string, repeated, tag = "1")]
        pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub items: ::prost::alloc::vec::Vec<MetaItems>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataItem {
        #[prost(string, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub items_str: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub stock: u32,
        #[prost(uint32, tag = "4")]
        pub price: u32,
        #[prost(uint32, tag = "5")]
        pub origin_price: u32,
        #[prost(int32, tag = "6")]
        pub sort: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsAttr {
    #[prost(string, tag = "1")]
    pub goods_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub sku: ::core::option::Option<GoodsSku>,
    #[prost(uint64, tag = "3")]
    pub ver: u64,
}
/// 查找品牌请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBrandRequest {
    /// 限定是否删除
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(oneof = "find_brand_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_brand_request::By>,
}
/// Nested message and enum types in `FindBrandRequest`.
pub mod find_brand_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 根据ID查找
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// 根据名称查找
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
/// 查找品牌响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBrandResponse {
    #[prost(message, optional, tag = "1")]
    pub brand: ::core::option::Option<Brand>,
}
/// 品牌列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
}
/// 品牌列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
/// 品牌是否存在请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandExistsRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// 分类的名称和父分类
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryNameAndParentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// 查找分类请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindCategoryRequest {
    /// 限定是否删除
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    /// 限定级别
    #[prost(enumeration = "CategoryLevel", optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    #[prost(oneof = "find_category_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_category_request::By>,
}
/// Nested message and enum types in `FindCategoryRequest`.
pub mod find_category_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 根据ID查找
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// 根据名称和父分类查找
        #[prost(message, tag = "2")]
        NameAndParent(super::CategoryNameAndParentRequest),
    }
}
/// 查找分类响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindCategoryResponse {
    #[prost(message, optional, tag = "1")]
    pub category: ::core::option::Option<Category>,
}
/// 分类列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "CategoryLevel", optional, tag = "5")]
    pub level: ::core::option::Option<i32>,
}
/// 分类列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub categoies: ::prost::alloc::vec::Vec<Category>,
}
/// 分类是否存在请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryExistsRequest {
    #[prost(message, optional, tag = "1")]
    pub name_and_parent: ::core::option::Option<CategoryNameAndParentRequest>,
    #[prost(string, optional, tag = "2")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// 分类树请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryTreeRequest {
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(enumeration = "CategoryLevel", optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof = "category_tree_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<category_tree_request::By>,
}
/// Nested message and enum types in `CategoryTreeRequest`.
pub mod category_tree_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 根据父ID查找直接子节点
        #[prost(string, tag = "1")]
        Parent(::prost::alloc::string::String),
        /// 根据path查找所有子节点
        #[prost(string, tag = "2")]
        Path(::prost::alloc::string::String),
    }
}
/// 分类树响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryTreeResponse {
    #[prost(message, repeated, tag = "1")]
    pub tree: ::prost::alloc::vec::Vec<CategoryTree>,
}
/// 查找带品牌信息分类请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindCategoryWithBrandsRequest {
    /// 限定是否删除
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    /// 限定级别
    #[prost(enumeration = "CategoryLevel", optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    /// 限定品牌名称
    #[prost(string, optional, tag = "5")]
    pub brand_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof = "find_category_with_brands_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_category_with_brands_request::By>,
}
/// Nested message and enum types in `FindCategoryWithBrandsRequest`.
pub mod find_category_with_brands_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 根据ID查找
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// 根据名称和父分类查找
        #[prost(message, tag = "2")]
        NameAndParent(super::CategoryNameAndParentRequest),
    }
}
/// 查找带品牌信息分类响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindCategoryWithBrandsResponse {
    #[prost(message, optional, tag = "1")]
    pub category: ::core::option::Option<CategoryWithBrands>,
}
/// 带品牌信息分类列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryWithBrandsRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "CategoryLevel", optional, tag = "5")]
    pub level: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub brand_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// 带品牌信息分类列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryWithBrandsResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub categoies: ::prost::alloc::vec::Vec<CategoryWithBrands>,
}
/// 带品牌信息的分类树响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryWithBrandsTreeResponse {
    #[prost(message, repeated, tag = "1")]
    pub tree: ::prost::alloc::vec::Vec<CategoryWithBrandsTree>,
}
/// 查找带分类信息品牌请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBrandWithCategoiesRequest {
    /// 限定是否删除
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    /// 限定分类名称
    #[prost(string, optional, tag = "4")]
    pub category_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof = "find_brand_with_categoies_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_brand_with_categoies_request::By>,
}
/// Nested message and enum types in `FindBrandWithCategoiesRequest`.
pub mod find_brand_with_categoies_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        /// 根据ID查找
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// 根据名称查找
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
/// 查找带分类信息品牌响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindBrandWithCategoiesResponse {
    #[prost(message, optional, tag = "1")]
    pub brand: ::core::option::Option<BrandWithCategoies>,
}
/// 带分类信息品牌列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandWithCategoiesRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    /// 限定分类名称
    #[prost(string, optional, tag = "4")]
    pub category_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// 带分类信息品牌列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandWithCategoiesResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub brands: ::prost::alloc::vec::Vec<BrandWithCategoies>,
}
/// 设置分类-品牌请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCategoryBrandsRequest {
    #[prost(string, tag = "1")]
    pub category_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub brand_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 清空分类的品牌请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCategoryBrandsRequest {
    #[prost(string, tag = "1")]
    pub category_id: ::prost::alloc::string::String,
}
/// 创建带品牌的分类请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCategoryWithBrandsRequest {
    #[prost(message, optional, tag = "1")]
    pub category: ::core::option::Option<Category>,
    #[prost(string, repeated, tag = "2")]
    pub brand_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 创建带品牌分类的响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCategoryWithBrandsResponse {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<Id>,
    #[prost(message, optional, tag = "2")]
    pub aff: ::core::option::Option<Aff>,
}
/// 查找商品请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindGoodsRequest {
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(oneof = "find_goods_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_goods_request::By>,
}
/// Nested message and enum types in `FindGoodsRequest`.
pub mod find_goods_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Sn(::prost::alloc::string::String),
    }
}
/// 查找商品响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindGoodsResponse {
    #[prost(message, optional, tag = "1")]
    pub goods: ::core::option::Option<Goods>,
}
/// 商品列表请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoodsRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub category_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub ship_fee_range: ::core::option::Option<U32Range>,
    #[prost(message, optional, tag = "7")]
    pub origin_price_range: ::core::option::Option<U32Range>,
    #[prost(message, optional, tag = "8")]
    pub price_range: ::core::option::Option<U32Range>,
    #[prost(message, optional, tag = "9")]
    pub date_range: ::core::option::Option<DateRange>,
    #[prost(bool, optional, tag = "10")]
    pub is_sale: ::core::option::Option<bool>,
    #[prost(enumeration = "ListGoodsOrder", optional, tag = "11")]
    pub primary_order: ::core::option::Option<i32>,
    #[prost(enumeration = "ListGoodsOrder", optional, tag = "12")]
    pub secondary_order: ::core::option::Option<i32>,
}
/// 商品列表响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoodsResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub goods: ::prost::alloc::vec::Vec<Goods>,
}
/// 商品是否存在请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsExistsRequest {
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof = "goods_exists_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<goods_exists_request::By>,
}
/// Nested message and enum types in `GoodsExistsRequest`.
pub mod goods_exists_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        #[prost(string, tag = "1")]
        Name(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Sn(::prost::alloc::string::String),
    }
}
/// 查找商品属性响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindGoodsAttrResponse {
    #[prost(message, optional, tag = "1")]
    pub goods_attr: ::core::option::Option<GoodsAttr>,
}
/// 更新商品库存请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoodsStockRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sku_key: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub increment: i32,
    #[prost(uint64, tag = "4")]
    pub ver: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ListGoodsOrder {
    ById = 0,
    ByIsNew = 1,
    ByIsHot = 2,
    ByHit = 3,
    BySoldNum = 4,
    ByShipFee = 5,
    ByOriginPrice = 6,
    ByPrice = 7,
    ByIsSale = 8,
}
impl ListGoodsOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ListGoodsOrder::ById => "BY_ID",
            ListGoodsOrder::ByIsNew => "BY_IS_NEW",
            ListGoodsOrder::ByIsHot => "BY_IS_HOT",
            ListGoodsOrder::ByHit => "BY_HIT",
            ListGoodsOrder::BySoldNum => "BY_SOLD_NUM",
            ListGoodsOrder::ByShipFee => "BY_SHIP_FEE",
            ListGoodsOrder::ByOriginPrice => "BY_ORIGIN_PRICE",
            ListGoodsOrder::ByPrice => "BY_PRICE",
            ListGoodsOrder::ByIsSale => "BY_IS_SALE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BY_ID" => Some(Self::ById),
            "BY_IS_NEW" => Some(Self::ByIsNew),
            "BY_IS_HOT" => Some(Self::ByIsHot),
            "BY_HIT" => Some(Self::ByHit),
            "BY_SOLD_NUM" => Some(Self::BySoldNum),
            "BY_SHIP_FEE" => Some(Self::ByShipFee),
            "BY_ORIGIN_PRICE" => Some(Self::ByOriginPrice),
            "BY_PRICE" => Some(Self::ByPrice),
            "BY_IS_SALE" => Some(Self::ByIsSale),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod goods_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 商品服务
    #[derive(Debug, Clone)]
    pub struct GoodsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GoodsServiceClient<tonic::transport::Channel> {
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
    impl<T> GoodsServiceClient<T>
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
        ) -> GoodsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GoodsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建品牌
        pub async fn create_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::Brand>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CreateBrand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CreateBrand"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改品牌
        pub async fn edit_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::Brand>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/EditBrand",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "EditBrand"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除或还原品牌
        pub async fn delete_or_restore_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/DeleteOrRestoreBrand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "DeleteOrRestoreBrand"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找品牌
        pub async fn find_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::FindBrandRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBrandResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindBrand",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "FindBrand"));
            self.inner.unary(req, path, codec).await
        }
        /// 品牌列表
        pub async fn list_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBrandRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBrandResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ListBrand",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "ListBrand"));
            self.inner.unary(req, path, codec).await
        }
        /// 品牌是否存在
        pub async fn brand_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::BrandExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/BrandExists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "BrandExists"));
            self.inner.unary(req, path, codec).await
        }
        /// 创建分类
        pub async fn create_category(
            &mut self,
            request: impl tonic::IntoRequest<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CreateCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CreateCategory"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改分类
        pub async fn edit_category(
            &mut self,
            request: impl tonic::IntoRequest<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/EditCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "EditCategory"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改分类名称
        pub async fn edit_category_name(
            &mut self,
            request: impl tonic::IntoRequest<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/EditCategoryName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "EditCategoryName"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除或还原分类
        pub async fn delete_or_restore_category(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/DeleteOrRestoreCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "DeleteOrRestoreCategory"));
            self.inner.unary(req, path, codec).await
        }
        /// 分类是否存在
        pub async fn category_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::CategoryExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CategoryExists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CategoryExists"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找分类
        pub async fn find_category(
            &mut self,
            request: impl tonic::IntoRequest<super::FindCategoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindCategoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "FindCategory"));
            self.inner.unary(req, path, codec).await
        }
        /// 分类列表
        pub async fn list_category(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCategoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCategoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ListCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "ListCategory"));
            self.inner.unary(req, path, codec).await
        }
        /// 分类树
        pub async fn category_tree(
            &mut self,
            request: impl tonic::IntoRequest<super::CategoryTreeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CategoryTreeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CategoryTree",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CategoryTree"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找带品牌信息的分类
        pub async fn find_category_with_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::FindCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindCategoryWithBrandsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindCategoryWithBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "FindCategoryWithBrands"));
            self.inner.unary(req, path, codec).await
        }
        /// 带品牌信息分类列表
        pub async fn list_category_with_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCategoryWithBrandsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ListCategoryWithBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "ListCategoryWithBrands"));
            self.inner.unary(req, path, codec).await
        }
        /// 带品牌信息分类树
        pub async fn category_with_brands_tree(
            &mut self,
            request: impl tonic::IntoRequest<super::CategoryTreeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CategoryWithBrandsTreeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CategoryWithBrandsTree",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CategoryWithBrandsTree"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找带分类信息的品牌
        pub async fn find_brand_with_categoies(
            &mut self,
            request: impl tonic::IntoRequest<super::FindBrandWithCategoiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBrandWithCategoiesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindBrandWithCategoies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "FindBrandWithCategoies"));
            self.inner.unary(req, path, codec).await
        }
        /// 带分类信息品牌列表
        pub async fn list_brand_with_categoies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBrandWithCategoiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBrandWithCategoiesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ListBrandWithCategoies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "ListBrandWithCategoies"));
            self.inner.unary(req, path, codec).await
        }
        /// 设置分类-品牌
        pub async fn set_category_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCategoryBrandsRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/SetCategoryBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "SetCategoryBrands"));
            self.inner.unary(req, path, codec).await
        }
        /// 清空分类的品牌
        pub async fn clear_category_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearCategoryBrandsRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ClearCategoryBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "ClearCategoryBrands"));
            self.inner.unary(req, path, codec).await
        }
        /// 创建带品牌的分类
        pub async fn create_category_with_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCategoryWithBrandsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CreateCategoryWithBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CreateCategoryWithBrands"));
            self.inner.unary(req, path, codec).await
        }
        /// 创建商品
        pub async fn create_goods(
            &mut self,
            request: impl tonic::IntoRequest<super::Goods>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/CreateGoods",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "CreateGoods"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改商品
        pub async fn edit_goods(
            &mut self,
            request: impl tonic::IntoRequest<super::Goods>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/EditGoods",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "EditGoods"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除/还原商品
        pub async fn delete_or_restore_goods(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/DeleteOrRestoreGoods",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "DeleteOrRestoreGoods"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找商品
        pub async fn find_goods(
            &mut self,
            request: impl tonic::IntoRequest<super::FindGoodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindGoodsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindGoods",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "FindGoods"));
            self.inner.unary(req, path, codec).await
        }
        /// 商品列表
        pub async fn list_goods(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGoodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGoodsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/ListGoods",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.GoodsService", "ListGoods"));
            self.inner.unary(req, path, codec).await
        }
        /// 商品是否存在
        pub async fn goods_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::GoodsExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/GoodsExists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "GoodsExists"));
            self.inner.unary(req, path, codec).await
        }
        /// 设置商品属性
        pub async fn set_goods_attr(
            &mut self,
            request: impl tonic::IntoRequest<super::GoodsAttr>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/SetGoodsAttr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "SetGoodsAttr"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除商品属性
        pub async fn remove_goods_attr(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/RemoveGoodsAttr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "RemoveGoodsAttr"));
            self.inner.unary(req, path, codec).await
        }
        /// 查找商品属性
        pub async fn find_goods_attr(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> std::result::Result<
            tonic::Response<super::FindGoodsAttrResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/FindGoodsAttr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "FindGoodsAttr"));
            self.inner.unary(req, path, codec).await
        }
        /// 更新商品库存
        pub async fn update_goods_stock(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoodsStockRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.GoodsService/UpdateGoodsStock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.GoodsService", "UpdateGoodsStock"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod goods_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GoodsServiceServer.
    #[async_trait]
    pub trait GoodsService: Send + Sync + 'static {
        /// 创建品牌
        async fn create_brand(
            &self,
            request: tonic::Request<super::Brand>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        /// 修改品牌
        async fn edit_brand(
            &self,
            request: tonic::Request<super::Brand>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除或还原品牌
        async fn delete_or_restore_brand(
            &self,
            request: tonic::Request<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 查找品牌
        async fn find_brand(
            &self,
            request: tonic::Request<super::FindBrandRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBrandResponse>,
            tonic::Status,
        >;
        /// 品牌列表
        async fn list_brand(
            &self,
            request: tonic::Request<super::ListBrandRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBrandResponse>,
            tonic::Status,
        >;
        /// 品牌是否存在
        async fn brand_exists(
            &self,
            request: tonic::Request<super::BrandExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        >;
        /// 创建分类
        async fn create_category(
            &self,
            request: tonic::Request<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        /// 修改分类
        async fn edit_category(
            &self,
            request: tonic::Request<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 修改分类名称
        async fn edit_category_name(
            &self,
            request: tonic::Request<super::Category>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除或还原分类
        async fn delete_or_restore_category(
            &self,
            request: tonic::Request<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 分类是否存在
        async fn category_exists(
            &self,
            request: tonic::Request<super::CategoryExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        >;
        /// 查找分类
        async fn find_category(
            &self,
            request: tonic::Request<super::FindCategoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindCategoryResponse>,
            tonic::Status,
        >;
        /// 分类列表
        async fn list_category(
            &self,
            request: tonic::Request<super::ListCategoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCategoryResponse>,
            tonic::Status,
        >;
        /// 分类树
        async fn category_tree(
            &self,
            request: tonic::Request<super::CategoryTreeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CategoryTreeResponse>,
            tonic::Status,
        >;
        /// 查找带品牌信息的分类
        async fn find_category_with_brands(
            &self,
            request: tonic::Request<super::FindCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindCategoryWithBrandsResponse>,
            tonic::Status,
        >;
        /// 带品牌信息分类列表
        async fn list_category_with_brands(
            &self,
            request: tonic::Request<super::ListCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCategoryWithBrandsResponse>,
            tonic::Status,
        >;
        /// 带品牌信息分类树
        async fn category_with_brands_tree(
            &self,
            request: tonic::Request<super::CategoryTreeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CategoryWithBrandsTreeResponse>,
            tonic::Status,
        >;
        /// 查找带分类信息的品牌
        async fn find_brand_with_categoies(
            &self,
            request: tonic::Request<super::FindBrandWithCategoiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindBrandWithCategoiesResponse>,
            tonic::Status,
        >;
        /// 带分类信息品牌列表
        async fn list_brand_with_categoies(
            &self,
            request: tonic::Request<super::ListBrandWithCategoiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBrandWithCategoiesResponse>,
            tonic::Status,
        >;
        /// 设置分类-品牌
        async fn set_category_brands(
            &self,
            request: tonic::Request<super::SetCategoryBrandsRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 清空分类的品牌
        async fn clear_category_brands(
            &self,
            request: tonic::Request<super::ClearCategoryBrandsRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 创建带品牌的分类
        async fn create_category_with_brands(
            &self,
            request: tonic::Request<super::CreateCategoryWithBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCategoryWithBrandsResponse>,
            tonic::Status,
        >;
        /// 创建商品
        async fn create_goods(
            &self,
            request: tonic::Request<super::Goods>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        /// 修改商品
        async fn edit_goods(
            &self,
            request: tonic::Request<super::Goods>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除/还原商品
        async fn delete_or_restore_goods(
            &self,
            request: tonic::Request<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 查找商品
        async fn find_goods(
            &self,
            request: tonic::Request<super::FindGoodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindGoodsResponse>,
            tonic::Status,
        >;
        /// 商品列表
        async fn list_goods(
            &self,
            request: tonic::Request<super::ListGoodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGoodsResponse>,
            tonic::Status,
        >;
        /// 商品是否存在
        async fn goods_exists(
            &self,
            request: tonic::Request<super::GoodsExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsExistsResponse>,
            tonic::Status,
        >;
        /// 设置商品属性
        async fn set_goods_attr(
            &self,
            request: tonic::Request<super::GoodsAttr>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 删除商品属性
        async fn remove_goods_attr(
            &self,
            request: tonic::Request<super::Id>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        /// 查找商品属性
        async fn find_goods_attr(
            &self,
            request: tonic::Request<super::Id>,
        ) -> std::result::Result<
            tonic::Response<super::FindGoodsAttrResponse>,
            tonic::Status,
        >;
        /// 更新商品库存
        async fn update_goods_stock(
            &self,
            request: tonic::Request<super::UpdateGoodsStockRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
    }
    /// 商品服务
    #[derive(Debug)]
    pub struct GoodsServiceServer<T: GoodsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GoodsService> GoodsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GoodsServiceServer<T>
    where
        T: GoodsService,
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
                "/pb.GoodsService/CreateBrand" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBrandSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Brand>
                    for CreateBrandSvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Brand>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_brand(request).await
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
                        let method = CreateBrandSvc(inner);
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
                "/pb.GoodsService/EditBrand" => {
                    #[allow(non_camel_case_types)]
                    struct EditBrandSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Brand>
                    for EditBrandSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Brand>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).edit_brand(request).await };
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
                        let method = EditBrandSvc(inner);
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
                "/pb.GoodsService/DeleteOrRestoreBrand" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreBrandSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreRequest>
                    for DeleteOrRestoreBrandSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_brand(request).await
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
                        let method = DeleteOrRestoreBrandSvc(inner);
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
                "/pb.GoodsService/FindBrand" => {
                    #[allow(non_camel_case_types)]
                    struct FindBrandSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::FindBrandRequest>
                    for FindBrandSvc<T> {
                        type Response = super::FindBrandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindBrandRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).find_brand(request).await };
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
                        let method = FindBrandSvc(inner);
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
                "/pb.GoodsService/ListBrand" => {
                    #[allow(non_camel_case_types)]
                    struct ListBrandSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ListBrandRequest>
                    for ListBrandSvc<T> {
                        type Response = super::ListBrandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBrandRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_brand(request).await };
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
                        let method = ListBrandSvc(inner);
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
                "/pb.GoodsService/BrandExists" => {
                    #[allow(non_camel_case_types)]
                    struct BrandExistsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::BrandExistsRequest>
                    for BrandExistsSvc<T> {
                        type Response = super::IsExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BrandExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).brand_exists(request).await
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
                        let method = BrandExistsSvc(inner);
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
                "/pb.GoodsService/CreateCategory" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCategorySvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Category>
                    for CreateCategorySvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Category>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_category(request).await
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
                        let method = CreateCategorySvc(inner);
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
                "/pb.GoodsService/EditCategory" => {
                    #[allow(non_camel_case_types)]
                    struct EditCategorySvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Category>
                    for EditCategorySvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Category>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_category(request).await
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
                        let method = EditCategorySvc(inner);
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
                "/pb.GoodsService/EditCategoryName" => {
                    #[allow(non_camel_case_types)]
                    struct EditCategoryNameSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Category>
                    for EditCategoryNameSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Category>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_category_name(request).await
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
                        let method = EditCategoryNameSvc(inner);
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
                "/pb.GoodsService/DeleteOrRestoreCategory" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreCategorySvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreRequest>
                    for DeleteOrRestoreCategorySvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_category(request).await
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
                        let method = DeleteOrRestoreCategorySvc(inner);
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
                "/pb.GoodsService/CategoryExists" => {
                    #[allow(non_camel_case_types)]
                    struct CategoryExistsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::CategoryExistsRequest>
                    for CategoryExistsSvc<T> {
                        type Response = super::IsExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CategoryExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).category_exists(request).await
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
                        let method = CategoryExistsSvc(inner);
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
                "/pb.GoodsService/FindCategory" => {
                    #[allow(non_camel_case_types)]
                    struct FindCategorySvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::FindCategoryRequest>
                    for FindCategorySvc<T> {
                        type Response = super::FindCategoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindCategoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).find_category(request).await
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
                        let method = FindCategorySvc(inner);
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
                "/pb.GoodsService/ListCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListCategorySvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ListCategoryRequest>
                    for ListCategorySvc<T> {
                        type Response = super::ListCategoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCategoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_category(request).await
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
                        let method = ListCategorySvc(inner);
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
                "/pb.GoodsService/CategoryTree" => {
                    #[allow(non_camel_case_types)]
                    struct CategoryTreeSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::CategoryTreeRequest>
                    for CategoryTreeSvc<T> {
                        type Response = super::CategoryTreeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CategoryTreeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).category_tree(request).await
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
                        let method = CategoryTreeSvc(inner);
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
                "/pb.GoodsService/FindCategoryWithBrands" => {
                    #[allow(non_camel_case_types)]
                    struct FindCategoryWithBrandsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::FindCategoryWithBrandsRequest>
                    for FindCategoryWithBrandsSvc<T> {
                        type Response = super::FindCategoryWithBrandsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindCategoryWithBrandsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).find_category_with_brands(request).await
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
                        let method = FindCategoryWithBrandsSvc(inner);
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
                "/pb.GoodsService/ListCategoryWithBrands" => {
                    #[allow(non_camel_case_types)]
                    struct ListCategoryWithBrandsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ListCategoryWithBrandsRequest>
                    for ListCategoryWithBrandsSvc<T> {
                        type Response = super::ListCategoryWithBrandsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCategoryWithBrandsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_category_with_brands(request).await
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
                        let method = ListCategoryWithBrandsSvc(inner);
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
                "/pb.GoodsService/CategoryWithBrandsTree" => {
                    #[allow(non_camel_case_types)]
                    struct CategoryWithBrandsTreeSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::CategoryTreeRequest>
                    for CategoryWithBrandsTreeSvc<T> {
                        type Response = super::CategoryWithBrandsTreeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CategoryTreeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).category_with_brands_tree(request).await
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
                        let method = CategoryWithBrandsTreeSvc(inner);
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
                "/pb.GoodsService/FindBrandWithCategoies" => {
                    #[allow(non_camel_case_types)]
                    struct FindBrandWithCategoiesSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::FindBrandWithCategoiesRequest>
                    for FindBrandWithCategoiesSvc<T> {
                        type Response = super::FindBrandWithCategoiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindBrandWithCategoiesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).find_brand_with_categoies(request).await
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
                        let method = FindBrandWithCategoiesSvc(inner);
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
                "/pb.GoodsService/ListBrandWithCategoies" => {
                    #[allow(non_camel_case_types)]
                    struct ListBrandWithCategoiesSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ListBrandWithCategoiesRequest>
                    for ListBrandWithCategoiesSvc<T> {
                        type Response = super::ListBrandWithCategoiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBrandWithCategoiesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_brand_with_categoies(request).await
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
                        let method = ListBrandWithCategoiesSvc(inner);
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
                "/pb.GoodsService/SetCategoryBrands" => {
                    #[allow(non_camel_case_types)]
                    struct SetCategoryBrandsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::SetCategoryBrandsRequest>
                    for SetCategoryBrandsSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCategoryBrandsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_category_brands(request).await
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
                        let method = SetCategoryBrandsSvc(inner);
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
                "/pb.GoodsService/ClearCategoryBrands" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCategoryBrandsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ClearCategoryBrandsRequest>
                    for ClearCategoryBrandsSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCategoryBrandsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).clear_category_brands(request).await
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
                        let method = ClearCategoryBrandsSvc(inner);
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
                "/pb.GoodsService/CreateCategoryWithBrands" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCategoryWithBrandsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::CreateCategoryWithBrandsRequest>
                    for CreateCategoryWithBrandsSvc<T> {
                        type Response = super::CreateCategoryWithBrandsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateCategoryWithBrandsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_category_with_brands(request).await
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
                        let method = CreateCategoryWithBrandsSvc(inner);
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
                "/pb.GoodsService/CreateGoods" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGoodsSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Goods>
                    for CreateGoodsSvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Goods>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_goods(request).await
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
                        let method = CreateGoodsSvc(inner);
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
                "/pb.GoodsService/EditGoods" => {
                    #[allow(non_camel_case_types)]
                    struct EditGoodsSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Goods>
                    for EditGoodsSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Goods>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).edit_goods(request).await };
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
                        let method = EditGoodsSvc(inner);
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
                "/pb.GoodsService/DeleteOrRestoreGoods" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreGoodsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreRequest>
                    for DeleteOrRestoreGoodsSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_goods(request).await
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
                        let method = DeleteOrRestoreGoodsSvc(inner);
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
                "/pb.GoodsService/FindGoods" => {
                    #[allow(non_camel_case_types)]
                    struct FindGoodsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::FindGoodsRequest>
                    for FindGoodsSvc<T> {
                        type Response = super::FindGoodsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindGoodsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).find_goods(request).await };
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
                        let method = FindGoodsSvc(inner);
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
                "/pb.GoodsService/ListGoods" => {
                    #[allow(non_camel_case_types)]
                    struct ListGoodsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::ListGoodsRequest>
                    for ListGoodsSvc<T> {
                        type Response = super::ListGoodsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGoodsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_goods(request).await };
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
                        let method = ListGoodsSvc(inner);
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
                "/pb.GoodsService/GoodsExists" => {
                    #[allow(non_camel_case_types)]
                    struct GoodsExistsSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::GoodsExistsRequest>
                    for GoodsExistsSvc<T> {
                        type Response = super::IsExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GoodsExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).goods_exists(request).await
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
                        let method = GoodsExistsSvc(inner);
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
                "/pb.GoodsService/SetGoodsAttr" => {
                    #[allow(non_camel_case_types)]
                    struct SetGoodsAttrSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::GoodsAttr>
                    for SetGoodsAttrSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GoodsAttr>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_goods_attr(request).await
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
                        let method = SetGoodsAttrSvc(inner);
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
                "/pb.GoodsService/RemoveGoodsAttr" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGoodsAttrSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Id>
                    for RemoveGoodsAttrSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_goods_attr(request).await
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
                        let method = RemoveGoodsAttrSvc(inner);
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
                "/pb.GoodsService/FindGoodsAttr" => {
                    #[allow(non_camel_case_types)]
                    struct FindGoodsAttrSvc<T: GoodsService>(pub Arc<T>);
                    impl<T: GoodsService> tonic::server::UnaryService<super::Id>
                    for FindGoodsAttrSvc<T> {
                        type Response = super::FindGoodsAttrResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).find_goods_attr(request).await
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
                        let method = FindGoodsAttrSvc(inner);
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
                "/pb.GoodsService/UpdateGoodsStock" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGoodsStockSvc<T: GoodsService>(pub Arc<T>);
                    impl<
                        T: GoodsService,
                    > tonic::server::UnaryService<super::UpdateGoodsStockRequest>
                    for UpdateGoodsStockSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateGoodsStockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_goods_stock(request).await
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
                        let method = UpdateGoodsStockSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GoodsService> Clone for GoodsServiceServer<T> {
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
    impl<T: GoodsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GoodsService> tonic::server::NamedService for GoodsServiceServer<T> {
        const NAME: &'static str = "pb.GoodsService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserExistsRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserExistsResponse {
    #[prost(bool, tag = "1")]
    pub is_exists: bool,
}
/// -- 修改状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUserStatusRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "UserStatus", tag = "2")]
    pub status: i32,
}
/// -- 修改密码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUserPasswordRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 新密码
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// 当前密码
    #[prost(string, optional, tag = "3")]
    pub current_password: ::core::option::Option<::prost::alloc::string::String>,
}
/// -- 查找用户
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindUserRequest {
    #[prost(enumeration = "UserStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(oneof = "find_user_request::By", tags = "1, 2")]
    pub by: ::core::option::Option<find_user_request::By>,
}
/// Nested message and enum types in `FindUserRequest`.
pub mod find_user_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum By {
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Email(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
/// -- 用户列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserRequest {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<PaginateRequest>,
    #[prost(string, optional, tag = "2")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "UserStatus", optional, tag = "4")]
    pub status: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_del: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub date_range: ::core::option::Option<DateRange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserResponse {
    #[prost(message, optional, tag = "1")]
    pub paginate: ::core::option::Option<Paginate>,
    #[prost(message, repeated, tag = "2")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
/// Generated client implementations.
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
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
    impl<T> UserServiceClient<T>
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
        ) -> UserServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::User>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.UserService/CreateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.UserService", "CreateUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn user_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::UserExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UserExistsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.UserService/UserExists",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.UserService", "UserExists"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn edit_user(
            &mut self,
            request: impl tonic::IntoRequest<super::User>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.UserService/EditUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.UserService", "EditUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_or_restore_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.UserService/DeleteOrRestoreUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.UserService", "DeleteOrRestoreUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_user_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUserStatusRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.UserService/ChangeUserStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.UserService", "ChangeUserStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_user_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUserPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.UserService/ChangeUserPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pb.UserService", "ChangeUserPassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn find_user(
            &mut self,
            request: impl tonic::IntoRequest<super::FindUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.UserService/FindUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.UserService", "FindUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.UserService/ListUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("pb.UserService", "ListUser"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        async fn create_user(
            &self,
            request: tonic::Request<super::User>,
        ) -> std::result::Result<tonic::Response<super::Id>, tonic::Status>;
        async fn user_exists(
            &self,
            request: tonic::Request<super::UserExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UserExistsResponse>,
            tonic::Status,
        >;
        async fn edit_user(
            &self,
            request: tonic::Request<super::User>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        async fn delete_or_restore_user(
            &self,
            request: tonic::Request<super::DeleteOrRestoreRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        async fn change_user_status(
            &self,
            request: tonic::Request<super::ChangeUserStatusRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        async fn change_user_password(
            &self,
            request: tonic::Request<super::ChangeUserPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::Aff>, tonic::Status>;
        async fn find_user(
            &self,
            request: tonic::Request<super::FindUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindUserResponse>,
            tonic::Status,
        >;
        async fn list_user(
            &self,
            request: tonic::Request<super::ListUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserServiceServer<T: UserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserService> UserServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
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
                "/pb.UserService/CreateUser" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<super::User>
                    for CreateUserSvc<T> {
                        type Response = super::Id;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::User>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_user(request).await };
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
                        let method = CreateUserSvc(inner);
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
                "/pb.UserService/UserExists" => {
                    #[allow(non_camel_case_types)]
                    struct UserExistsSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::UserExistsRequest>
                    for UserExistsSvc<T> {
                        type Response = super::UserExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).user_exists(request).await };
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
                        let method = UserExistsSvc(inner);
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
                "/pb.UserService/EditUser" => {
                    #[allow(non_camel_case_types)]
                    struct EditUserSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<super::User>
                    for EditUserSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::User>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).edit_user(request).await };
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
                        let method = EditUserSvc(inner);
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
                "/pb.UserService/DeleteOrRestoreUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrRestoreUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeleteOrRestoreRequest>
                    for DeleteOrRestoreUserSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrRestoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_or_restore_user(request).await
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
                        let method = DeleteOrRestoreUserSvc(inner);
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
                "/pb.UserService/ChangeUserStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUserStatusSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ChangeUserStatusRequest>
                    for ChangeUserStatusSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeUserStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_user_status(request).await
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
                        let method = ChangeUserStatusSvc(inner);
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
                "/pb.UserService/ChangeUserPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUserPasswordSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ChangeUserPasswordRequest>
                    for ChangeUserPasswordSvc<T> {
                        type Response = super::Aff;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeUserPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_user_password(request).await
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
                        let method = ChangeUserPasswordSvc(inner);
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
                "/pb.UserService/FindUser" => {
                    #[allow(non_camel_case_types)]
                    struct FindUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::FindUserRequest>
                    for FindUserSvc<T> {
                        type Response = super::FindUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).find_user(request).await };
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
                        let method = FindUserSvc(inner);
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
                "/pb.UserService/ListUser" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ListUserRequest>
                    for ListUserSvc<T> {
                        type Response = super::ListUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_user(request).await };
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
                        let method = ListUserSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UserService> Clone for UserServiceServer<T> {
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
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "pb.UserService";
    }
}
