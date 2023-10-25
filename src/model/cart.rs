use serde::{Deserialize, Serialize};

use super::U32;
use crate::{
    pb,
    utils::{self, dt},
};

/// 购物车
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Cart {
    pub id: String,
    pub user_id: String,
    pub goods_id: String,
    pub goods_sku: String,
    pub num: U32,
    pub amount: U32,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub hash: String,
}

impl Cart {
    // 计算 hash 值
    pub fn hash(&self, id: &str) -> String {
        let data = format!("{}-{}-{}", &self.user_id, &self.goods_id, &self.goods_sku);
        utils::hash::md5(&data).unwrap_or(format!("{}-{}", id, &data)[..32].to_string())
    }

    pub fn into_pb_vec(cs: Vec<Self>) -> Vec<pb::Cart> {
        let mut v = Vec::with_capacity(cs.len());
        for c in cs {
            v.push(c.into());
        }
        v
    }

    pub fn from_pb_vc(cs: Vec<pb::Cart>) -> Vec<Self> {
        let mut v = Vec::with_capacity(cs.len());
        for c in cs {
            v.push(Self::from(c));
        }
        v
    }
}

impl From<pb::Cart> for Cart {
    fn from(c: pb::Cart) -> Self {
        Self {
            id: c.id,
            user_id: c.user_id,
            goods_id: c.goods_id,
            goods_sku: c.goods_sku,
            num: c.num.into(),
            amount: c.amount.into(),
            dateline: dt::prost2chrono(&c.dateline),
            hash: c.hash,
        }
    }
}

impl Into<pb::Cart> for Cart {
    fn into(self) -> pb::Cart {
        pb::Cart {
            id: self.id,
            user_id: self.user_id,
            goods_id: self.goods_id,
            goods_sku: self.goods_sku,
            num: self.num.unsigned(),
            amount: self.amount.unsigned(),
            dateline: dt::chrono2prost(&self.dateline),
            hash: self.hash,
        }
    }
}

// --- 购物车数量 ---
pub struct CartItemNum {
    pub id: String,
    pub num: U32,
}

impl From<pb::CartItemNum> for CartItemNum {
    fn from(cin: pb::CartItemNum) -> Self {
        Self {
            id: cin.id,
            num: cin.num.into(),
        }
    }
}

impl Into<pb::CartItemNum> for CartItemNum {
    fn into(self) -> pb::CartItemNum {
        pb::CartItemNum {
            id: self.id,
            num: self.num.unsigned(),
        }
    }
}

// -- 从购物车删除请求 --

pub struct RemoveItemFromCartRequest {
    pub user_id: String,
    pub ids: Vec<String>,
}

impl From<pb::RemoveItemFromCartRequest> for RemoveItemFromCartRequest {
    fn from(r: pb::RemoveItemFromCartRequest) -> Self {
        Self {
            user_id: r.user_id,
            ids: r.ids,
        }
    }
}

impl Into<pb::RemoveItemFromCartRequest> for RemoveItemFromCartRequest {
    fn into(self) -> pb::RemoveItemFromCartRequest {
        pb::RemoveItemFromCartRequest {
            user_id: self.user_id,
            ids: self.ids,
        }
    }
}
