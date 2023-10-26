use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

use super::{DatetimeRange, U32Range, U32};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "order_status")]
// #[repr(i32)]
pub enum OrderStatus {
    /// 未知
    #[default]
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

impl From<pb::OrderStatus> for OrderStatus {
    fn from(s: pb::OrderStatus) -> Self {
        match s {
            pb::OrderStatus::Unspecified => Self::Unspecified,
            pb::OrderStatus::PendingPay => Self::PendingPay,
            pb::OrderStatus::UserCancel => Self::UserCancel,
            pb::OrderStatus::AdminCancel => Self::AdminCancel,
            pb::OrderStatus::TimeoutCancel => Self::TimeoutCancel,
            pb::OrderStatus::Paied => Self::Paied,
            pb::OrderStatus::Delivering => Self::Delivering,
            pb::OrderStatus::Delivered => Self::Delivered,
            pb::OrderStatus::UserConfirmedDone => Self::UserConfirmedDone,
            pb::OrderStatus::AutoConfirmedDone => Self::AutoConfirmedDone,
        }
    }
}

impl Into<pb::OrderStatus> for OrderStatus {
    fn into(self) -> pb::OrderStatus {
        match self {
            OrderStatus::Unspecified => pb::OrderStatus::Unspecified,
            OrderStatus::PendingPay => pb::OrderStatus::PendingPay,
            OrderStatus::UserCancel => pb::OrderStatus::UserCancel,
            OrderStatus::AdminCancel => pb::OrderStatus::AdminCancel,
            OrderStatus::TimeoutCancel => pb::OrderStatus::TimeoutCancel,
            OrderStatus::Paied => pb::OrderStatus::Paied,
            OrderStatus::Delivering => pb::OrderStatus::Delivering,
            OrderStatus::Delivered => pb::OrderStatus::Delivered,
            OrderStatus::UserConfirmedDone => pb::OrderStatus::UserConfirmedDone,
            OrderStatus::AutoConfirmedDone => pb::OrderStatus::AutoConfirmedDone,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Order {
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 购物车ID
    pub cart_id: String,
    /// 编号
    pub sn: String,
    /// 状态
    pub status: OrderStatus,
    /// 金额
    pub amount: U32,
    /// 总数
    pub total_num: U32,
    /// 运费
    pub freight: U32,
    /// 收货地址
    pub address: sqlx::types::Json<super::AddressDetail>,
    /// 快递单号
    pub delivery_id: String,
    /// 下单时间
    pub dateline: chrono::DateTime<chrono::Local>,
    /// 自动取消订单的时间
    pub cancel_until_dateline: chrono::DateTime<chrono::Local>,
    /// 自动确认订单的时间
    pub confirm_until_dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

impl Order {
    pub fn to_pb_vec(v: Vec<Self>) -> Vec<pb::Order> {
        let mut ls = Vec::with_capacity(v.len());
        for o in v {
            ls.push(o.into());
        }
        ls
    }
    pub fn from_pb_vec(v: Vec<pb::Order>) -> Vec<Self> {
        let mut ls = Vec::with_capacity(v.len());
        for o in v {
            ls.push(o.into());
        }
        ls
    }

    pub fn to_pb_option(o: Option<Self>) -> Option<pb::Order> {
        match o {
            Some(o) => Some(o.into()),
            None => None,
        }
    }
    pub fn from_pb_option(o: Option<pb::Order>) -> Option<Self> {
        match o {
            Some(o) => Some(o.into()),
            None => None,
        }
    }
}

impl From<pb::Order> for Order {
    fn from(o: pb::Order) -> Self {
        let status = OrderStatus::from(
            pb::OrderStatus::from_i32(o.status).unwrap_or(pb::OrderStatus::Unspecified),
        );
        let addr = super::AddressDetail::from(o.address.unwrap());
        Self {
            id: o.id,
            user_id: o.user_id,
            cart_id: o.cart_id,
            sn: o.sn,
            status,
            amount: o.amount.into(),
            total_num: o.total_num.into(),
            freight: o.freight.into(),
            address: sqlx::types::Json::from(addr),
            delivery_id: o.delivery_id,
            dateline: dt::prost2chrono(&o.dateline),
            cancel_until_dateline: dt::prost2chrono(&o.cancel_until_dateline),
            confirm_until_dateline: dt::prost2chrono(&o.confirm_until_dateline),
            is_del: o.is_del,
        }
    }
}

impl Into<pb::Order> for Order {
    fn into(self) -> pb::Order {
        let status: pb::OrderStatus = self.status.into();
        let status = status.into();

        pb::Order {
            id: self.id,
            user_id: self.user_id,
            cart_id: self.cart_id,
            sn: self.sn,
            status,
            amount: self.amount.unsigned(),
            total_num: self.total_num.unsigned(),
            freight: self.freight.unsigned(),
            address: Some(self.address.0.into()),
            delivery_id: self.delivery_id,
            dateline: dt::chrono2prost(&self.dateline),
            cancel_until_dateline: dt::chrono2prost(&self.cancel_until_dateline),
            confirm_until_dateline: dt::chrono2prost(&self.confirm_until_dateline),
            is_del: self.is_del,
        }
    }
}

// --- 修改订单金额请求 --
pub struct EditOrderAmountRequest {
    pub id: String,
    pub amount: U32,
}

impl From<pb::EditOrderAmountRequest> for EditOrderAmountRequest {
    fn from(r: pb::EditOrderAmountRequest) -> Self {
        Self {
            id: r.id,
            amount: r.amount.into(),
        }
    }
}

impl Into<pb::EditOrderAmountRequest> for EditOrderAmountRequest {
    fn into(self) -> pb::EditOrderAmountRequest {
        pb::EditOrderAmountRequest {
            id: self.id,
            amount: self.amount.unsigned(),
        }
    }
}

// --- 修改订单收货地址请求 --

pub struct EditOrderAddressRequest {
    pub id: String,
    pub address: sqlx::types::Json<super::AddressDetail>,
}

impl From<pb::EditOrderAddressRequest> for EditOrderAddressRequest {
    fn from(r: pb::EditOrderAddressRequest) -> Self {
        let addr = super::AddressDetail::from(r.address.unwrap());
        Self {
            id: r.id,
            address: sqlx::types::Json::from(addr),
        }
    }
}

impl Into<pb::EditOrderAddressRequest> for EditOrderAddressRequest {
    fn into(self) -> pb::EditOrderAddressRequest {
        let addr: pb::AddressDetail = self.address.0.into();
        pb::EditOrderAddressRequest {
            id: self.id,
            address: Some(addr),
        }
    }
}

// -- 修改订单状态请求 --
pub struct EditOrderStatusRequest {
    pub id: String,
    pub status: OrderStatus,
    /// 前置状态
    pub pre_status: Option<OrderStatus>,
}

impl From<pb::EditOrderStatusRequest> for EditOrderStatusRequest {
    fn from(r: pb::EditOrderStatusRequest) -> Self {
        let status = pb::OrderStatus::from_i32(r.status).unwrap_or(pb::OrderStatus::Unspecified);
        let status = OrderStatus::from(status);

        let pre_status = match r.pre_status {
            Some(i) => match pb::OrderStatus::from_i32(i) {
                Some(s) => Some(OrderStatus::from(s)),
                None => None,
            },
            None => None,
        };

        Self {
            id: r.id,
            status,
            pre_status,
        }
    }
}

impl Into<pb::EditOrderStatusRequest> for EditOrderStatusRequest {
    fn into(self) -> pb::EditOrderStatusRequest {
        let status: pb::OrderStatus = self.status.into();
        let status = status.into();

        let pre_status = match self.pre_status {
            Some(s) => {
                let s: pb::OrderStatus = s.into();
                Some(s.into())
            }
            None => None,
        };

        pb::EditOrderStatusRequest {
            id: self.id,
            status,
            pre_status,
        }
    }
}

// -- 查找订单请求 --
pub enum FindOrderBy {
    ID(String),
    SN(String),
}

impl From<pb::find_order_request::By> for FindOrderBy {
    fn from(by: pb::find_order_request::By) -> Self {
        match by {
            pb::find_order_request::By::Id(id) => Self::ID(id),
            pb::find_order_request::By::Sn(sn) => Self::SN(sn),
        }
    }
}

impl Into<pb::find_order_request::By> for FindOrderBy {
    fn into(self) -> pb::find_order_request::By {
        match self {
            FindOrderBy::ID(id) => pb::find_order_request::By::Id(id),
            FindOrderBy::SN(sn) => pb::find_order_request::By::Sn(sn),
        }
    }
}

pub struct FindOrderRequest {
    pub by: FindOrderBy,
    pub user_id: Option<String>,
    pub is_del: Option<bool>,
}

impl From<pb::FindOrderRequest> for FindOrderRequest {
    fn from(r: pb::FindOrderRequest) -> Self {
        let by = FindOrderBy::from(r.by.unwrap());
        Self {
            by,
            user_id: r.user_id,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindOrderRequest> for FindOrderRequest {
    fn into(self) -> pb::FindOrderRequest {
        pb::FindOrderRequest {
            is_del: self.is_del,
            user_id: self.user_id,
            by: Some(self.by.into()),
        }
    }
}

// --  订单列表请求 --

pub struct ListOrderRequest {
    pub paginate: db::PaginateRequest,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    pub user_id: Option<String>,
    pub consignee: Option<String>,
    pub phone: Option<String>,
    /// 详细地址
    pub address: Option<String>,
    pub is_del: Option<bool>,
    /// 订单编号
    pub sn: Option<String>,
    /// 状态
    pub status: Option<OrderStatus>,
    /// 快递单号
    pub delivery_id: Option<String>,
    /// 下单时间区间
    pub date_range: Option<DatetimeRange>,
    /// 取消时间区间
    pub cancel_date_range: Option<DatetimeRange>,
    /// 确认时间区间
    pub confirm_date_range: Option<DatetimeRange>,
    /// 金额区间
    pub amount_range: Option<U32Range>,
}

impl From<pb::ListOrderRequest> for ListOrderRequest {
    fn from(r: pb::ListOrderRequest) -> Self {
        let status = match r.status {
            Some(s) => match pb::OrderStatus::from_i32(s) {
                Some(s) => Some(s.into()),
                None => None,
            },
            None => None,
        };

        let date_range = match r.date_range {
            Some(dr) => Some(DatetimeRange::from(dr)),
            None => None,
        };
        let cancel_date_range: Option<DatetimeRange> = match r.cancel_date_range {
            Some(dr) => Some(DatetimeRange::from(dr)),
            None => None,
        };
        let confirm_date_range: Option<DatetimeRange> = match r.confirm_date_range {
            Some(dr) => Some(DatetimeRange::from(dr)),
            None => None,
        };
        let amount_range = match r.amount_range {
            Some(ar) => Some(U32Range::from(ar)),
            None => None,
        };

        Self {
            paginate: r.paginate.unwrap().into(),
            user_id: r.user_id,
            consignee: r.consignee,
            phone: r.phone,
            address: r.address,
            is_del: r.is_del,
            sn: r.sn,
            status,
            delivery_id: r.delivery_id,
            date_range,
            cancel_date_range,
            confirm_date_range,
            amount_range,
        }
    }
}

impl Into<pb::ListOrderRequest> for ListOrderRequest {
    fn into(self) -> pb::ListOrderRequest {
        let status = match self.status {
            Some(s) => {
                let s: pb::OrderStatus = s.into();
                Some(s.into())
            }
            None => None,
        };

        let date_range = match self.date_range {
            Some(dr) => Some(dr.into()),
            None => None,
        };
        let cancel_date_range: Option<pb::DateRange> = match self.cancel_date_range {
            Some(dr) => Some(dr.into()),
            None => None,
        };
        let confirm_date_range: Option<pb::DateRange> = match self.confirm_date_range {
            Some(dr) => Some(dr.into()),
            None => None,
        };
        let amount_range = match self.amount_range {
            Some(ar) => Some(ar.into()),
            None => None,
        };

        pb::ListOrderRequest {
            paginate: Some(self.paginate.into()),
            user_id: self.user_id,
            consignee: self.consignee,
            phone: self.phone,
            address: self.address,
            is_del: self.is_del,
            sn: self.sn,
            status,
            delivery_id: self.delivery_id,
            date_range,
            cancel_date_range,
            confirm_date_range,
            amount_range,
        }
    }
}
