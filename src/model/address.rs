use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct AddressDetail {
    /// 收件人
    pub consignee: String,
    /// 电话
    pub phone: String,
    /// 详细地址
    pub address: String,
    /// 省市区【省级】
    pub province: String,
    /// 城市【地级】
    pub city: String,
    /// 县市区【县级】
    pub county: String,
    /// 邮编
    pub post_code: String,
}

impl From<pb::AddressDetail> for AddressDetail {
    fn from(d: pb::AddressDetail) -> Self {
        Self {
            consignee: d.consignee,
            phone: d.phone,
            address: d.address,
            province: d.province,
            city: d.city,
            county: d.county,
            post_code: d.post_code,
        }
    }
}

impl Into<pb::AddressDetail> for AddressDetail {
    fn into(self) -> pb::AddressDetail {
        pb::AddressDetail {
            consignee: self.consignee,
            phone: self.phone,
            address: self.address,
            province: self.province,
            city: self.city,
            county: self.county,
            post_code: self.post_code,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Address {
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 是否默认地址
    pub is_default: bool,
    /// 是否删除
    pub is_del: bool,
    /// 添加时间
    pub dateline: chrono::DateTime<chrono::Local>,
    /// 地址详情
    #[sqlx(flatten)]
    pub detail: AddressDetail,
}

impl From<pb::Address> for Address {
    fn from(a: pb::Address) -> Self {
        Self {
            id: a.id,
            user_id: a.user_id,
            is_default: a.is_default,
            is_del: a.is_del,
            dateline: dt::prost2chrono(&a.dateline),
            detail: a.detail.unwrap().into(),
        }
    }
}

impl Into<pb::Address> for Address {
    fn into(self) -> pb::Address {
        pb::Address {
            id: self.id,
            user_id: self.user_id,
            is_default: self.is_default,
            is_del: self.is_del,
            dateline: dt::chrono2prost(&self.dateline),
            detail: Some(self.detail.into()),
        }
    }
}

// -- 查找地址请求 --

pub enum FindAddressBy {
    ID(String),
    IsDefault,
}

impl From<pb::find_address_request::By> for FindAddressBy {
    fn from(by: pb::find_address_request::By) -> Self {
        match by {
            pb::find_address_request::By::Id(id) => Self::ID(id),
            pb::find_address_request::By::IsDefault(_) => Self::IsDefault,
        }
    }
}

impl Into<pb::find_address_request::By> for FindAddressBy {
    fn into(self) -> pb::find_address_request::By {
        match self {
            Self::ID(id) => pb::find_address_request::By::Id(id),
            Self::IsDefault => pb::find_address_request::By::IsDefault(true),
        }
    }
}

pub struct FindAddressRequest {
    pub by: FindAddressBy,
    pub user_id: Option<String>,
    pub is_del: Option<bool>,
}

impl From<pb::FindAddressRequest> for FindAddressRequest {
    fn from(r: pb::FindAddressRequest) -> Self {
        Self {
            by: FindAddressBy::from(r.by.unwrap_or(pb::find_address_request::By::IsDefault(true))),
            user_id: r.user_id,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindAddressRequest> for FindAddressRequest {
    fn into(self) -> pb::FindAddressRequest {
        pb::FindAddressRequest {
            user_id: self.user_id,
            is_del: self.is_del,
            by: Some(self.by.into()),
        }
    }
}

// -- 地址列表请求 --

pub struct ListAddressRequest {
    pub paginate: db::PaginateRequest,
    pub user_id: Option<String>,
    pub consignee: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_del: Option<bool>,
}

impl From<pb::ListAddressRequest> for ListAddressRequest {
    fn from(r: pb::ListAddressRequest) -> Self {
        Self {
            paginate: r.paginate.unwrap().into(),
            user_id: r.user_id,
            consignee: r.consignee,
            phone: r.phone,
            address: r.address,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::ListAddressRequest> for ListAddressRequest {
    fn into(self) -> pb::ListAddressRequest {
        pb::ListAddressRequest {
            paginate: Some(self.paginate.into()),
            user_id: self.user_id,
            consignee: self.consignee,
            phone: self.phone,
            address: self.address,
            is_del: self.is_del,
        }
    }
}
