use std::sync::Arc;

use super::err_to_tonic_status as ce2s;
use crate::{
    db, model,
    pb::{self, banner_service_server::BannerService},
};

pub struct Banner {
    pool: Arc<sqlx::PgPool>,
}

impl Banner {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl BannerService for Banner {
    /// 创建轮播图
    async fn create_banner(
        &self,
        request: tonic::Request<pb::Banner>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let b = model::Banner::from(request.into_inner());
        let id = db::banner::create(&self.pool, &b).await.map_err(ce2s)?;
        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改轮播图
    async fn edit_banner(
        &self,
        request: tonic::Request<pb::Banner>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let b = model::Banner::from(request.into_inner());

        if b.id.is_empty() {
            return Err(tonic::Status::invalid_argument("请指定要修改的轮播图ID"));
        }

        let rows = db::banner::edit(&self.pool, &b).await.map_err(ce2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除/还原轮播图
    async fn delete_or_restore_banner(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();

        if r.id.is_empty() {
            return Err(tonic::Status::invalid_argument(
                "请指定要删除/还原的轮播图ID",
            ));
        }

        let rows = db::banner::del_or_restore(&self.pool, r.id, r.is_del)
            .await
            .map_err(ce2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找轮播图
    async fn find_banner(
        &self,
        request: tonic::Request<pb::FindBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBannerResponse>, tonic::Status> {
        let r = model::FindBannerRequest::from(request.into_inner());
        let b = db::banner::find(&self.pool, &r).await.map_err(ce2s)?;

        let banner = match b {
            Some(b) => Some(b.into()),
            None => None,
        };

        Ok(tonic::Response::new(pb::FindBannerResponse { banner }))
    }
    /// 轮播图列表
    async fn list_banner(
        &self,
        request: tonic::Request<pb::ListBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListBannerResponse>, tonic::Status> {
        let r = model::ListBannerRequest::from(request.into_inner());
        let p = db::banner::list(&self.pool, &r).await.map_err(ce2s)?;

        let paginate = p.to_pb();

        let mut banners = Vec::with_capacity(p.data.len());
        for b in p.data {
            banners.push(b.into());
        }
        Ok(tonic::Response::new(pb::ListBannerResponse {
            paginate: Some(paginate),
            banners,
        }))
    }
    /// 前N个轮播图
    async fn top_n_banner(
        &self,
        request: tonic::Request<pb::TopNBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::TopNBannerResponse>, tonic::Status> {
        let r = model::TopNBannerRequest::from(request.into_inner());

        let ls = db::banner::top(&self.pool, &r).await.map_err(ce2s)?;

        let mut banners = Vec::with_capacity(ls.len());
        for b in ls {
            banners.push(b.into());
        }

        Ok(tonic::Response::new(pb::TopNBannerResponse { banners }))
    }
}
