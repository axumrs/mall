use std::sync::Arc;

use super::{err_to_tonic_status as ce2s, sqlx_err_to_tonic_status as e2s};

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
        let id = db::brand::create(&self.pool, &m).await.map_err(ce2s)?;

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

        let rows = db::brand::edit(&self.pool, &m).await.map_err(ce2s)?;

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
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找品牌
    async fn find_brand(
        &self,
        request: tonic::Request<pb::FindBrandRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBrandResponse>, tonic::Status> {
        let r = request.into_inner();
        let r = model::BrandFindRequest::from(r);
        let brand = db::brand::find(&self.pool, &r).await.map_err(ce2s)?;
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
        let rs = db::brand::list(&self.pool, &r).await.map_err(ce2s)?;
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
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: rs }))
    }
    /// 创建分类
    async fn create_category(
        &self,
        request: tonic::Request<pb::Category>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let cate = model::Category::from(request.into_inner());
        let exists_req = model::CategoryExistsRequest {
            name_and_parent: model::CategoryNameAndParentRequest {
                name: cate.name.clone(),
                parent: cate.parent.clone(),
            },
            id: None,
        };

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let exists = match db::category::exists(&mut *tx, &exists_req).await {
            Ok(exists) => exists,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        if exists {
            tx.rollback().await.map_err(e2s)?;
            return Err(tonic::Status::already_exists("同名的分类已存在"));
        }

        let id = match db::category::create(&mut *tx, &cate).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改分类
    async fn edit_category(
        &self,
        request: tonic::Request<pb::Category>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        unimplemented!()
    }
    /// 删除或还原分类
    async fn delete_or_restore_category(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        unimplemented!()
    }
    /// 分类是否存在
    async fn category_exists(
        &self,
        request: tonic::Request<pb::CategoryExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::IsExistsResponse>, tonic::Status> {
        let r = request.into_inner();
        {
            let r_check = r.name_and_parent.clone();
            let e_check = Err(tonic::Status::invalid_argument(
                "参数错误：请至少指定分类名称；如果父分类为空，则视为一级分类",
            ));
            if r_check.is_none() {
                return e_check;
            }
            if r_check.unwrap().name.is_empty() {
                return e_check;
            }
        }
        let r = model::CategoryExistsRequest::from(r);
        let mut tx = self.pool.begin().await.map_err(e2s)?;
        let exists = match db::category::exists(&mut *tx, &r).await {
            Err(err) => {
                tx.rollback().await.map_err(e2s)?;

                return Err(e2s(err));
            }
            Ok(exists) => exists,
        };
        tx.commit().await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: exists }))
    }
}
