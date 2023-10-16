use serde::{Deserialize, Serialize};

use crate::{pb, utils::dt};

use super::{U32, U64};

pub type Money = U32;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Goods {
    pub id: String,
    pub category_id: String,
    pub name: String,
    pub sn: String,
    pub is_sale: bool,
    pub ship_fee: Money,
    pub is_new: bool,
    pub is_hot: bool,
    pub hit: U64,
    pub sold_num: U64,
    pub fav_num: U64,
    pub origin_price: Money,
    pub price: Money,
    pub brief: String,
    pub cover: String,
    pub images: Vec<String>,
    pub description: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

impl From<pb::Goods> for Goods {
    fn from(g: pb::Goods) -> Self {
        Self {
            id: g.id,
            category_id: g.category_id,
            name: g.name,
            sn: g.sn,
            is_sale: g.is_sale,
            ship_fee: g.ship_fee.into(),
            is_new: g.is_new,
            is_hot: g.is_hot,
            hit: g.hit.into(),
            sold_num: g.sold_num.into(),
            fav_num: g.fav_num.into(),
            origin_price: g.origin_price.into(),
            price: g.price.into(),
            brief: g.brief,
            cover: g.cover,
            images: g.images,
            description: g.description,
            dateline: dt::prost2chrono(&g.dateline),
            is_del: g.is_del,
        }
    }
}

impl Into<pb::Goods> for Goods {
    fn into(self) -> pb::Goods {
        pb::Goods {
            id: self.id,
            category_id: self.category_id,
            name: self.name,
            sn: self.sn,
            is_sale: self.is_sale,
            ship_fee: self.ship_fee.into(),
            is_new: self.is_new,
            is_hot: self.is_hot,
            hit: self.hit.into(),
            sold_num: self.sold_num.into(),
            fav_num: self.fav_num.into(),
            origin_price: self.origin_price.into(),
            price: self.price.into(),
            brief: self.brief,
            cover: self.cover,
            images: self.images,
            description: self.description,
            dateline: dt::chrono2prost(&self.dateline),
            is_del: self.is_del,
        }
    }
}
