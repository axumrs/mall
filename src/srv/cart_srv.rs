use std::sync::Arc;

use super::sqlx_err_to_tonic_status as e2s;
use crate::{
    db, model,
    pb::{self, cart_service_server::CartService},
};
pub struct Cart {
    pool: Arc<sqlx::PgPool>,
}

impl Cart {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

async fn _get_cart<'a>(
    e: impl sqlx::PgExecutor<'a>,
    user_id: &'a str,
) -> Result<Vec<pb::Cart>, sqlx::Error> {
    let ls = db::cart::get(e, user_id).await?;
    Ok(model::Cart::into_pb_vec(ls))
}

#[tonic::async_trait]
impl CartService for Cart {
    /// 添加到购物车
    async fn add_item_to_cart(
        &self,
        request: tonic::Request<pb::Cart>,
    ) -> std::result::Result<tonic::Response<pb::AddItemToCartResponse>, tonic::Status> {
        let item = model::Cart::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let id = match db::cart::add_item(&mut *tx, &item).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        let ls = match _get_cart(&mut *tx, &item.user_id).await {
            Ok(ls) => ls,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::AddItemToCartResponse {
            detail: Some(pb::CartDetailResponse { cart: ls }),
            id: Some(pb::Id { value: id }),
        }))
    }
    /// 获取购物车
    async fn get_cart(
        &self,
        request: tonic::Request<pb::GetCartRequest>,
    ) -> std::result::Result<tonic::Response<pb::CartDetailResponse>, tonic::Status> {
        let r = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(e2s)?;
        let ls = match _get_cart(&mut *tx, &r.user_id).await {
            Ok(ls) => ls,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::CartDetailResponse { cart: ls }))
    }
    /// 清空购物车
    async fn clear_cart(
        &self,
        request: tonic::Request<pb::ClearCartRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();

        let rows = db::cart::clear(&*self.pool, &r.user_id)
            .await
            .map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 从购物车删除
    async fn remove_item_from_cart(
        &self,
        request: tonic::Request<pb::RemoveItemFromCartRequest>,
    ) -> std::result::Result<tonic::Response<pb::RemoveItemFromCartResponse>, tonic::Status> {
        let r = model::RemoveItemFromCartRequest::from(request.into_inner());

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let removed = match db::cart::remove_item(&mut *tx, &r).await {
            Ok(ls) => ls,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        let ls = match _get_cart(&mut *tx, &r.user_id).await {
            Ok(ls) => ls,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::RemoveItemFromCartResponse {
            detail: Some(pb::CartDetailResponse { cart: ls }),
            removed: Some(pb::CartDetailResponse {
                cart: model::Cart::into_pb_vec(removed),
            }),
        }))
    }
    /// 更新购物车某项的数量
    async fn update_cart_item_num(
        &self,
        request: tonic::Request<pb::UpdateCartItemNumRequest>,
    ) -> std::result::Result<tonic::Response<pb::UpdateCartItemNumResponse>, tonic::Status> {
        let r = request.into_inner();

        let items_len = r.items.len();
        let user_id = r.user_id.as_str();

        let mut items = Vec::with_capacity(items_len);
        for r in r.items {
            items.push(model::CartItemNum::from(r));
        }

        let mut affs = Vec::with_capacity(items_len);

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        for item in items.iter() {
            let aff = match db::cart::update_num(&mut *tx, item, user_id).await {
                Ok(aff) => aff,
                Err(e) => {
                    tx.rollback().await.map_err(e2s)?;
                    return Err(e2s(e));
                }
            };
            affs.push(aff);
        }

        let ls = match _get_cart(&mut *tx, user_id).await {
            Ok(ls) => ls,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::UpdateCartItemNumResponse {
            detail: Some(pb::CartDetailResponse { cart: ls }),
            affs: affs
                .iter()
                .map(|a| pb::Aff { rows: a.to_owned() })
                .collect::<Vec<pb::Aff>>(),
        }))
    }
}
