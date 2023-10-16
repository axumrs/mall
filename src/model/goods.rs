use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

use super::{DatetimeRange, U32Range, U32, U64};

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

// --- 是否存在请求 ---
pub enum GoodsExistsBy {
    Name(String),
    SN(String),
}

pub struct GoodsExistsRequest {
    pub by: GoodsExistsBy,
    pub id: Option<String>,
}

impl From<pb::GoodsExistsRequest> for GoodsExistsRequest {
    fn from(r: pb::GoodsExistsRequest) -> Self {
        let by = match r.by.unwrap() {
            pb::goods_exists_request::By::Name(name) => GoodsExistsBy::Name(name),
            pb::goods_exists_request::By::Sn(sn) => GoodsExistsBy::SN(sn),
        };
        Self { by, id: r.id }
    }
}

impl Into<pb::GoodsExistsRequest> for GoodsExistsRequest {
    fn into(self) -> pb::GoodsExistsRequest {
        let by = match self.by {
            GoodsExistsBy::Name(name) => pb::goods_exists_request::By::Name(name),
            GoodsExistsBy::SN(sn) => pb::goods_exists_request::By::Sn(sn),
        };
        pb::GoodsExistsRequest {
            id: self.id,
            by: Some(by),
        }
    }
}

// -- 查找商品请求 --

pub enum FindGoodsBy {
    ID(String),
    SN(String),
}

pub struct FindGoodsRequest {
    pub by: FindGoodsBy,
    pub is_del: Option<bool>,
}

impl From<pb::FindGoodsRequest> for FindGoodsRequest {
    fn from(r: pb::FindGoodsRequest) -> Self {
        let by = match r.by.unwrap() {
            pb::find_goods_request::By::Id(id) => FindGoodsBy::ID(id),
            pb::find_goods_request::By::Sn(sn) => FindGoodsBy::SN(sn),
        };
        Self {
            by,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindGoodsRequest> for FindGoodsRequest {
    fn into(self) -> pb::FindGoodsRequest {
        let by = match self.by {
            FindGoodsBy::ID(id) => pb::find_goods_request::By::Id(id),
            FindGoodsBy::SN(sn) => pb::find_goods_request::By::Sn(sn),
        };
        pb::FindGoodsRequest {
            is_del: self.is_del,
            by: Some(by),
        }
    }
}

// --- 商品列表请求 ---

#[derive(Default)]
pub enum ListGoodsOrder {
    #[default]
    ById,
    ByIsNew,
    ByIsHot,
    ByHit,
    BySoldNum,
    ByShipFee,
    ByOriginPrice,
    ByPrice,
    ByIsSale,
}

impl From<pb::ListGoodsOrder> for ListGoodsOrder {
    fn from(o: pb::ListGoodsOrder) -> Self {
        match o {
            pb::ListGoodsOrder::ById => Self::ById,
            pb::ListGoodsOrder::ByIsNew => Self::ByIsNew,
            pb::ListGoodsOrder::ByIsHot => Self::ByIsHot,
            pb::ListGoodsOrder::ByHit => Self::ByHit,
            pb::ListGoodsOrder::BySoldNum => Self::BySoldNum,
            pb::ListGoodsOrder::ByShipFee => Self::ByShipFee,
            pb::ListGoodsOrder::ByOriginPrice => Self::ByOriginPrice,
            pb::ListGoodsOrder::ByPrice => Self::ByPrice,
            pb::ListGoodsOrder::ByIsSale => Self::ByIsSale,
        }
    }
}

impl Into<pb::ListGoodsOrder> for ListGoodsOrder {
    fn into(self) -> pb::ListGoodsOrder {
        match self {
            Self::ById => pb::ListGoodsOrder::ById,
            Self::ByIsNew => pb::ListGoodsOrder::ByIsNew,
            Self::ByIsHot => pb::ListGoodsOrder::ByIsHot,
            Self::ByHit => pb::ListGoodsOrder::ByHit,
            Self::BySoldNum => pb::ListGoodsOrder::BySoldNum,
            Self::ByShipFee => pb::ListGoodsOrder::ByShipFee,
            Self::ByOriginPrice => pb::ListGoodsOrder::ByOriginPrice,
            Self::ByPrice => pb::ListGoodsOrder::ByPrice,
            Self::ByIsSale => pb::ListGoodsOrder::ByIsSale,
        }
    }
}

pub struct ListGoodsRequest {
    pub paginate: db::PaginateRequest,
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub sn: Option<String>,
    pub category_id: Option<String>,
    pub ship_fee_range: Option<U32Range>,
    pub origin_price_range: Option<U32Range>,
    pub price_range: Option<U32Range>,
    pub date_range: Option<DatetimeRange>,
    pub is_sale: Option<bool>,
    pub primary_order: Option<ListGoodsOrder>,
    pub secondary_order: Option<ListGoodsOrder>,
}

impl From<pb::ListGoodsRequest> for ListGoodsRequest {
    fn from(r: pb::ListGoodsRequest) -> Self {
        let ship_fee_range = match r.ship_fee_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let origin_price_range = match r.origin_price_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let price_range = match r.price_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let date_range = match r.date_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };

        let primary_order = match r.primary_order {
            Some(i) => Some(pb::ListGoodsOrder::from_i32(i).unwrap().into()),
            None => None,
        };
        let secondary_order = match r.secondary_order {
            Some(i) => Some(pb::ListGoodsOrder::from_i32(i).unwrap().into()),
            None => None,
        };

        Self {
            paginate: r.paginate.unwrap().into(),
            name: r.name,
            is_del: r.is_del,
            sn: r.sn,
            category_id: r.category_id,
            ship_fee_range,
            origin_price_range,
            price_range,
            date_range,
            is_sale: r.is_sale,
            primary_order,
            secondary_order,
        }
    }
}

impl Into<pb::ListGoodsRequest> for ListGoodsRequest {
    fn into(self) -> pb::ListGoodsRequest {
        let ship_fee_range = match self.ship_fee_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let origin_price_range = match self.origin_price_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let price_range = match self.price_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let date_range: Option<pb::DateRange> = match self.date_range {
            Some(rg) => Some(rg.into()),
            None => None,
        };
        let primary_order = match self.primary_order {
            Some(o) => {
                let o: pb::ListGoodsOrder = o.into();
                Some(o.into())
            }
            None => None,
        };
        let secondary_order = match self.secondary_order {
            Some(o) => {
                let o: pb::ListGoodsOrder = o.into();
                Some(o.into())
            }
            None => None,
        };

        pb::ListGoodsRequest {
            paginate: Some(self.paginate.into()),
            name: self.name,
            is_del: self.is_del,
            sn: self.sn,
            category_id: self.category_id,
            ship_fee_range,
            origin_price_range,
            price_range,
            date_range,
            is_sale: self.is_sale,
            primary_order,
            secondary_order,
        }
    }
}
