use std::sync::Arc;

use super::err_to_tonic_status as ce2s;
use super::sqlx_err_to_tonic_status as e2s;
use crate::{
    db, model,
    pb::{self, order_service_server::OrderService},
};
pub struct Order {
    pool: Arc<sqlx::PgPool>,
}

impl Order {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl OrderService for Order {
    /// 添加地址
    async fn create_address(
        &self,
        request: tonic::Request<pb::Address>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let addr = model::Address::from(request.into_inner());

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        // 是否有默认地址
        let has_default = match db::address::has_default(&mut *tx, &addr.user_id).await {
            Ok(hd) => hd,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        let addr = model::Address {
            is_default: !has_default,
            ..addr
        };

        // 插入
        let id = match db::address::create(&mut *tx, &addr).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改地址
    async fn edit_address(
        &self,
        request: tonic::Request<pb::Address>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let addr = model::Address::from(request.into_inner());

        if addr.id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要修改的ID"));
        }

        let rows = db::address::edit(&*self.pool, &addr).await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除/还原地址
    async fn delete_or_restore_address(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreAddressRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        if r.dos.is_none() {
            return Err(tonic::Status::invalid_argument("未指定要操作的数据"));
        }
        let dos = r.dos.unwrap();

        if dos.id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要操作的ID"));
        }

        let rows = db::address::del_or_restore(&*self.pool, &dos.id, dos.is_del, r.user_id)
            .await
            .map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找地址
    async fn find_address(
        &self,
        request: tonic::Request<pb::FindAddressRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindAddressResponse>, tonic::Status> {
        let r = model::FindAddressRequest::from(request.into_inner());
        let addr = db::address::find(&*self.pool, &r).await.map_err(e2s)?;

        let addr = match addr {
            Some(addr) => Some(addr.into()),
            None => None,
        };

        Ok(tonic::Response::new(pb::FindAddressResponse {
            address: addr,
        }))
    }
    /// 地址列表
    async fn list_address(
        &self,
        request: tonic::Request<pb::ListAddressRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListAddressResponse>, tonic::Status> {
        let r = model::ListAddressRequest::from(request.into_inner());
        let p = db::address::list(&*self.pool, &r).await.map_err(ce2s)?;
        let paginate = p.to_pb();

        let mut data = Vec::with_capacity(p.data.len());
        for addr in p.data {
            data.push(addr.into());
        }

        Ok(tonic::Response::new(pb::ListAddressResponse {
            paginate: Some(paginate),
            address_list: data,
        }))
    }
    /// 设置默认地址
    async fn set_default_address(
        &self,
        request: tonic::Request<pb::Address>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let addr = model::Address::from(request.into_inner());

        if addr.id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要操作的ID"));
        }

        if addr.user_id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要操作的用户ID"));
        }

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        // 清空默认地址
        if let Err(e) = db::address::clear_default(&mut *tx, &addr.user_id).await {
            tx.rollback().await.map_err(e2s)?;
            return Err(e2s(e));
        }

        // 设置默认地址
        let rows = match db::address::set_default(&mut *tx, &addr.id, &addr.user_id).await {
            Ok(aff) => aff,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 获取默认地址
    async fn get_default_address(
        &self,
        request: tonic::Request<pb::GetDefaultAddressRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindAddressResponse>, tonic::Status> {
        let r = request.into_inner();
        if r.user_id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要操作的用户ID"));
        }

        let addr = db::address::get_default(&*self.pool, &r.user_id)
            .await
            .map_err(e2s)?;

        let addr = match addr {
            Some(addr) => Some(addr.into()),
            None => None,
        };

        Ok(tonic::Response::new(pb::FindAddressResponse {
            address: addr,
        }))
    }
    /// 创建订单
    async fn create_order(
        &self,
        request: tonic::Request<pb::Order>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let order = model::Order::from(request.into_inner());
        let id = db::order::create(&*self.pool, &order).await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改订单金额
    async fn edit_order_amount(
        &self,
        request: tonic::Request<pb::EditOrderAmountRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::EditOrderAmountRequest::from(request.into_inner());
        let rows = db::order::edit_amount(&*self.pool, &r).await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 修改订单收货地址
    async fn edit_order_address(
        &self,
        request: tonic::Request<pb::EditOrderAddressRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::EditOrderAddressRequest::from(request.into_inner());
        let rows = db::order::edit_address(&*self.pool, &r)
            .await
            .map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 修改订单状态
    async fn edit_order_status(
        &self,
        request: tonic::Request<pb::EditOrderStatusRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::EditOrderStatusRequest::from(request.into_inner());
        let rows = db::order::edit_status(&*self.pool, &r).await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除或还原订单
    async fn delete_or_restore_order(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreOrderRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let dor = r.dor.unwrap();

        if dor.id.is_empty() {
            return Err(tonic::Status::invalid_argument("未指定要操作的ID"));
        }

        let rows = db::order::del_or_restore(&*self.pool, &dor.id, dor.is_del, r.user_id)
            .await
            .map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找订单
    async fn find_order(
        &self,
        request: tonic::Request<pb::FindOrderRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindOrderResponse>, tonic::Status> {
        let r = model::FindOrderRequest::from(request.into_inner());
        let order = db::order::find(&*self.pool, &r).await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::FindOrderResponse {
            order: model::Order::to_pb_option(order),
        }))
    }
    /// 订单列表
    async fn list_order(
        &self,
        request: tonic::Request<pb::ListOrderRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListOrderResponse>, tonic::Status> {
        let r = model::ListOrderRequest::from(request.into_inner());
        let p = db::order::list(&*self.pool, &r).await.map_err(ce2s)?;
        Ok(tonic::Response::new(pb::ListOrderResponse {
            paginate: Some(p.to_pb()),
            orders: model::Order::to_pb_vec(p.data),
        }))
    }
    /// 创建订单商品
    async fn create_order_goods(
        &self,
        request: tonic::Request<pb::OrderGoods>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let og = model::OrderGoods::from(request.into_inner());
        let id = db::order_goods::create(&*self.pool, &og)
            .await
            .map_err(e2s)?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
}
