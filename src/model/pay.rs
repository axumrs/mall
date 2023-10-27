use serde::{Deserialize, Serialize};

use crate::{pb, utils::dt};

use super::U32;

/// 支付状态
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "pay_status")]
pub enum PayStatus {
    #[default]
    /// 未指定
    Unspecified,
    /// 未支付
    Unpay,
    /// 正在支付
    Paying,
    /// 支付完成
    Done,
    /// 超时取消
    TimeoutCancel,
}

impl From<pb::PayStatus> for PayStatus {
    fn from(s: pb::PayStatus) -> Self {
        match s {
            pb::PayStatus::PaystatusUnspecified => Self::Unspecified,
            pb::PayStatus::PaystatusUnpay => Self::Unpay,
            pb::PayStatus::PaystatusPaying => Self::Paying,
            pb::PayStatus::PaystatusDone => Self::Done,
            pb::PayStatus::PaystatusTimeoutCancel => Self::TimeoutCancel,
        }
    }
}

impl Into<pb::PayStatus> for PayStatus {
    fn into(self) -> pb::PayStatus {
        match self {
            PayStatus::Unspecified => pb::PayStatus::PaystatusUnspecified,
            PayStatus::Unpay => pb::PayStatus::PaystatusUnpay,
            PayStatus::Paying => pb::PayStatus::PaystatusPaying,
            PayStatus::Done => pb::PayStatus::PaystatusDone,
            PayStatus::TimeoutCancel => pb::PayStatus::PaystatusTimeoutCancel,
        }
    }
}

/// 支付
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Pay {
    pub id: String,
    /// 订单ID
    pub order_id: String,
    /// 状态
    pub status: PayStatus,
    /// 交易号
    pub tx_id: String,
    /// 支付金额
    pub amount: U32,
    /// 创建时间
    pub dateline: chrono::DateTime<chrono::Local>,
    /// 超时时间
    pub timeout_until_dateline: chrono::DateTime<chrono::Local>,
    /// 完成时间
    pub done_dateline: chrono::DateTime<chrono::Local>,
}

impl From<pb::Pay> for Pay {
    fn from(p: pb::Pay) -> Self {
        let status =
            pb::PayStatus::from_i32(p.status).unwrap_or(pb::PayStatus::PaystatusUnspecified);
        let status = status.into();
        Self {
            id: p.id,
            order_id: p.order_id,
            status,
            tx_id: p.tx_id,
            amount: p.amount.into(),
            dateline: dt::prost2chrono(&p.dateline),
            timeout_until_dateline: dt::prost2chrono(&p.timeout_until_dateline),
            done_dateline: dt::prost2chrono(&p.done_dateline),
        }
    }
}

impl Into<pb::Pay> for Pay {
    fn into(self) -> pb::Pay {
        let status: pb::PayStatus = self.status.into();
        pb::Pay {
            id: self.id,
            order_id: self.order_id,
            status: status.into(),
            tx_id: self.tx_id,
            amount: self.amount.unsigned(),
            dateline: dt::chrono2prost(&self.dateline),
            timeout_until_dateline: dt::chrono2prost(&self.timeout_until_dateline),
            done_dateline: dt::chrono2prost(&self.done_dateline),
        }
    }
}

// -- 查找支付请求 --
pub struct FindPayRequest {
    /// 支付ID
    pub id: String,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    pub user_id: Option<String>,
}

impl From<pb::FindPayRequest> for FindPayRequest {
    fn from(r: pb::FindPayRequest) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
        }
    }
}

impl Into<pb::FindPayRequest> for FindPayRequest {
    fn into(self) -> pb::FindPayRequest {
        pb::FindPayRequest {
            id: self.id,
            user_id: self.user_id,
        }
    }
}

// -- 订单支付列表请求 --
pub struct ListPayForOrderRequest {
    /// 订单ID
    pub order_id: String,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    pub user_id: Option<String>,
    /// 过滤支付状态
    pub status: Option<PayStatus>,
    /// 显示最新的N条记录
    pub top: Option<i32>,
    /// 是否将已完成支付的记录放在列表开始
    pub pined_done: bool,
}

impl From<pb::ListPayForOrderRequest> for ListPayForOrderRequest {
    fn from(r: pb::ListPayForOrderRequest) -> Self {
        let status = match r.status {
            Some(i) => match pb::PayStatus::from_i32(i) {
                Some(s) => Some(s.into()),
                None => None,
            },
            None => None,
        };
        Self {
            order_id: r.order_id,
            user_id: r.user_id,
            status,
            top: r.top,
            pined_done: r.pined_done,
        }
    }
}

impl Into<pb::ListPayForOrderRequest> for ListPayForOrderRequest {
    fn into(self) -> pb::ListPayForOrderRequest {
        let status = match self.status {
            Some(s) => {
                let s: pb::PayStatus = s.into();
                Some(s.into())
            }
            None => None,
        };
        pb::ListPayForOrderRequest {
            order_id: self.order_id,
            user_id: self.user_id,
            status,
            top: self.top,
            pined_done: self.pined_done,
        }
    }
}

// -- 订单支付列表响应 --
pub struct ListPayForOrderResponse {
    /// 列表数量
    pub total: i64,
    /// 列表
    pub pays: Vec<Pay>,
}

impl From<pb::ListPayForOrderResponse> for ListPayForOrderResponse {
    fn from(r: pb::ListPayForOrderResponse) -> Self {
        let mut pays = Vec::with_capacity(r.pays.len());
        for p in r.pays {
            pays.push(p.into());
        }

        Self {
            total: r.total,
            pays,
        }
    }
}

impl Into<pb::ListPayForOrderResponse> for ListPayForOrderResponse {
    fn into(self) -> pb::ListPayForOrderResponse {
        let mut pays = Vec::with_capacity(self.pays.len());
        for p in self.pays {
            pays.push(p.into())
        }

        pb::ListPayForOrderResponse {
            total: self.total,
            pays,
        }
    }
}

// -- 完成支付请求 --

pub struct PayDoneRequest {
    /// 支付ID
    pub id: String,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    pub user_id: Option<String>,
    /// 交易ID
    pub tx_id: String,
}

impl From<pb::PayDoneRequest> for PayDoneRequest {
    fn from(r: pb::PayDoneRequest) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            tx_id: r.tx_id,
        }
    }
}

impl Into<pb::PayDoneRequest> for PayDoneRequest {
    fn into(self) -> pb::PayDoneRequest {
        pb::PayDoneRequest {
            id: self.id,
            user_id: self.user_id,
            tx_id: self.tx_id,
        }
    }
}

// -- 是否正在支付请求 --
pub struct OrderIsPayingRequest {
    /// 订单ID
    pub order_id: String,
    /// 用户ID。如果是用户进行操作，必须指定该参数
    pub user_id: Option<String>,
}

impl From<pb::OrderIsPayingRequest> for OrderIsPayingRequest {
    fn from(r: pb::OrderIsPayingRequest) -> Self {
        Self {
            order_id: r.order_id,
            user_id: r.user_id,
        }
    }
}

impl Into<pb::OrderIsPayingRequest> for OrderIsPayingRequest {
    fn into(self) -> pb::OrderIsPayingRequest {
        pb::OrderIsPayingRequest {
            order_id: self.order_id,
            user_id: self.user_id,
        }
    }
}
