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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aff {
    #[prost(uint64, tag = "1")]
    pub rows: u64,
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
pub struct Id {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
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
/// 是否存在
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsExistsResponse {
    #[prost(bool, tag = "1")]
    pub value: bool,
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
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
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
