use std::sync::Arc;

use crate::pb::{self, banner_service_server::BannerService};

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
        unimplemented!()
    }
    /// 修改轮播图
    async fn edit_banner(
        &self,
        request: tonic::Request<pb::Banner>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        unimplemented!()
    }
    /// 删除/还原轮播图
    async fn delete_or_restore_banner(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        unimplemented!()
    }
    /// 查找轮播图
    async fn find_banner(
        &self,
        request: tonic::Request<pb::FindBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBannerResponse>, tonic::Status> {
        unimplemented!()
    }
    /// 轮播图列表
    async fn list_banner(
        &self,
        request: tonic::Request<pb::ListBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListBannerResponse>, tonic::Status> {
        unimplemented!()
    }
    /// 前N个轮播图
    async fn top_n_banner(
        &self,
        request: tonic::Request<pb::TopNBannerRequest>,
    ) -> std::result::Result<tonic::Response<pb::TopNBannerResponse>, tonic::Status> {
        unimplemented!()
    }
}
