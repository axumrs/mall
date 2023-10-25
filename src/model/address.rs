use serde::{Deserialize, Serialize};

use crate::{pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
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

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
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
