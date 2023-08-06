use std::sync::Arc;

use crate::{
    db, model,
    pb::{self, goods_service_server::GoodsService},
};

pub struct Goods {
    pool: Arc<sqlx::PgPool>,
}

impl Goods {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl GoodsService for Goods {
    /// 创建品牌
    async fn create_brand(
        &self,
        request: tonic::Request<pb::Brand>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let r = request.into_inner();
        let m = model::Brand::from(r);

        let is_exists = self
            .brand_exists(tonic::Request::new(pb::BrandExistsRequest {
                name: m.name.clone(),
                id: None,
            }))
            .await?
            .into_inner();
        if is_exists.value {
            return Err(tonic::Status::already_exists("品牌已存在"));
        };
        let id = db::brand::create(&self.pool, &m)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改品牌
    async fn edit_brand(
        &self,
        request: tonic::Request<pb::Brand>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let id = r.id.clone();

        if (*id).is_empty() {
            return Err(tonic::Status::invalid_argument("请指定ID"));
        }

        let m = model::Brand::from(r);

        let is_exists = self
            .brand_exists(tonic::Request::new(pb::BrandExistsRequest {
                name: m.name.clone(),
                id: Some(id),
            }))
            .await?
            .into_inner();
        if is_exists.value {
            return Err(tonic::Status::already_exists("品牌已存在"));
        }

        let rows = db::brand::edit(&self.pool, &m)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除或还原品牌
    async fn delete_or_restore_brand(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let rows = db::brand::del_or_restore(&self.pool, r.id, r.is_del)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找品牌
    async fn find_brand(
        &self,
        request: tonic::Request<pb::FindBrandRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBrandResponse>, tonic::Status> {
        let r = request.into_inner();
        let r = model::BrandFindRequest::from(r);
        let brand = db::brand::find(&self.pool, &r)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        let brand: Option<pb::Brand> = match brand {
            Some(b) => Some(b.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindBrandResponse { brand }))
    }
    /// 品牌列表
    async fn list_brand(
        &self,
        request: tonic::Request<pb::ListBrandRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListBrandResponse>, tonic::Status> {
        let r = model::BrandListRequest::from(request.into_inner());
        let rs = db::brand::list(&self.pool, &r)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        let paginate = rs.to_pb();

        let mut brands = Vec::with_capacity(rs.data.len());
        for b in rs.data {
            brands.push(b.into());
        }
        Ok(tonic::Response::new(pb::ListBrandResponse {
            paginate: Some(paginate),
            brands,
        }))
    }
    /// 品牌是否存在
    async fn brand_exists(
        &self,
        request: tonic::Request<pb::BrandExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::IsExistsResponse>, tonic::Status> {
        let r = request.into_inner();
        let rs = db::brand::exists(&self.pool, &r.name, r.id)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: rs }))
    }
}
