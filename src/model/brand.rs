use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Brand {
    pub id: String,
    pub name: String,
    pub logo: String,
    pub is_del: bool,
    pub dateline: chrono::DateTime<chrono::Local>,
}

impl From<pb::Brand> for Brand {
    fn from(b: pb::Brand) -> Self {
        Self {
            id: b.id,
            name: b.name,
            logo: b.logo,
            is_del: b.is_del,
            dateline: dt::prost2chrono(&b.dateline),
        }
    }
}

impl Into<pb::Brand> for Brand {
    fn into(self) -> pb::Brand {
        pb::Brand {
            id: self.id,
            name: self.name,
            logo: self.logo,
            is_del: self.is_del,
            dateline: dt::chrono2prost(&self.dateline),
        }
    }
}

pub enum BrandFindBy {
    ID(String),
    Name(String),
}

impl From<pb::find_brand_request::By> for BrandFindBy {
    fn from(b: pb::find_brand_request::By) -> Self {
        match b {
            pb::find_brand_request::By::Id(id) => Self::ID(id),
            pb::find_brand_request::By::Name(name) => Self::Name(name),
        }
    }
}

impl Into<pb::find_brand_request::By> for BrandFindBy {
    fn into(self) -> pb::find_brand_request::By {
        match self {
            Self::ID(id) => pb::find_brand_request::By::Id(id),
            Self::Name(name) => pb::find_brand_request::By::Name(name),
        }
    }
}

pub struct BrandFindRequest {
    pub by: BrandFindBy,
    pub is_del: Option<bool>,
}

impl From<pb::FindBrandRequest> for BrandFindRequest {
    fn from(r: pb::FindBrandRequest) -> Self {
        let by = r.by.unwrap().into();
        Self {
            by,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindBrandRequest> for BrandFindRequest {
    fn into(self) -> pb::FindBrandRequest {
        pb::FindBrandRequest {
            is_del: self.is_del,
            by: Some(self.by.into()),
        }
    }
}

pub struct BrandListRequest {
    pub paginate: db::PaginateRequest,
    pub name: Option<String>,
    pub is_del: Option<bool>,
}

impl From<pb::ListBrandRequest> for BrandListRequest {
    fn from(r: pb::ListBrandRequest) -> Self {
        Self {
            paginate: db::PaginateRequest::from(r.paginate.unwrap_or_default()),
            name: r.name,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::ListBrandRequest> for BrandListRequest {
    fn into(self) -> pb::ListBrandRequest {
        pb::ListBrandRequest {
            paginate: Some(self.paginate.into()),
            name: self.name,
            is_del: self.is_del,
        }
    }
}
