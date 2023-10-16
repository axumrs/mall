use serde::{Deserialize, Serialize};

use crate::{db, pb};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Banner {
    pub id: String,
    pub img: String,
    pub url: String,
    pub sort: i32,
    pub title: String,
    pub is_del: bool,
}

impl From<pb::Banner> for Banner {
    fn from(b: pb::Banner) -> Self {
        Self {
            id: b.id,
            img: b.img,
            url: b.url,
            sort: b.sort,
            title: b.title,
            is_del: b.is_del,
        }
    }
}

impl Into<pb::Banner> for Banner {
    fn into(self) -> pb::Banner {
        pb::Banner {
            id: self.id,
            img: self.img,
            url: self.url,
            sort: self.sort,
            title: self.title,
            is_del: self.is_del,
        }
    }
}

// --- 查找请求 ---

#[derive(Default)]
pub struct FindBannerRequest {
    pub id: String,
    pub title: Option<String>,
    pub is_del: Option<bool>,
}

impl From<pb::FindBannerRequest> for FindBannerRequest {
    fn from(r: pb::FindBannerRequest) -> Self {
        Self {
            id: r.id,
            title: r.title,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindBannerRequest> for FindBannerRequest {
    fn into(self) -> pb::FindBannerRequest {
        pb::FindBannerRequest {
            id: self.id,
            title: self.title,
            is_del: self.is_del,
        }
    }
}

// --- 列表请求 ----
#[derive(Default)]
pub struct ListBannerRequest {
    pub paginate: db::PaginateRequest,
    pub title: Option<String>,
    pub is_del: Option<bool>,
    pub order_by_sort: bool,
}

impl From<pb::ListBannerRequest> for ListBannerRequest {
    fn from(r: pb::ListBannerRequest) -> Self {
        Self {
            paginate: r.paginate.unwrap().into(),
            title: r.title,
            is_del: r.is_del,
            order_by_sort: r.order_by_sort,
        }
    }
}

impl Into<pb::ListBannerRequest> for ListBannerRequest {
    fn into(self) -> pb::ListBannerRequest {
        pb::ListBannerRequest {
            paginate: Some(self.paginate.into()),
            title: self.title,
            is_del: self.is_del,
            order_by_sort: self.order_by_sort,
        }
    }
}

// -- 前N个请求
#[derive(Default)]
pub struct TopNBannerRequest {
    pub num: i32,
    pub order_by_id: bool,
    pub title: Option<String>,
}

impl From<pb::TopNBannerRequest> for TopNBannerRequest {
    fn from(r: pb::TopNBannerRequest) -> Self {
        Self {
            num: r.num,
            order_by_id: r.order_by_id,
            title: r.title,
        }
    }
}

impl Into<pb::TopNBannerRequest> for TopNBannerRequest {
    fn into(self) -> pb::TopNBannerRequest {
        pb::TopNBannerRequest {
            num: self.num,
            order_by_id: self.order_by_id,
            title: self.title,
        }
    }
}
