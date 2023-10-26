use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

use super::U32;

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
