use serde::{Deserialize, Serialize};

use super::U32;
use crate::pb;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct OrderGoods {
    pub id: String,
    pub order_id: String,
    pub goods_id: String,
    pub goods_sku: String,
    pub goods_snapshot: sqlx::types::Json<super::GoodsSKUData>,
    pub num: U32,
    pub price: U32,
}

impl From<pb::OrderGoods> for OrderGoods {
    fn from(g: pb::OrderGoods) -> Self {
        let goods_snapshot: super::GoodsSKUData = g.goods_snapshot.unwrap().into();
        Self {
            id: g.id,
            order_id: g.order_id,
            goods_id: g.goods_id,
            goods_sku: g.goods_sku,
            goods_snapshot: sqlx::types::Json::from(goods_snapshot),
            num: g.num.into(),
            price: g.price.into(),
        }
    }
}

impl Into<pb::OrderGoods> for OrderGoods {
    fn into(self) -> pb::OrderGoods {
        pb::OrderGoods {
            id: self.id,
            order_id: self.order_id,
            goods_id: self.goods_id,
            goods_sku: self.goods_sku,
            goods_snapshot: Some(self.goods_snapshot.0.into()),
            num: self.num.unsigned(),
            price: self.price.unsigned(),
        }
    }
}
