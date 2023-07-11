use crate::pb;

pub const DEFAULT_PAGE_SIZE: u32 = 30;
pub struct PaginateRequest {
    pub page: u32,
    pub page_size: u32,
}

impl std::default::Default for PaginateRequest {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl From<pb::PaginateRequest> for PaginateRequest {
    fn from(pr: pb::PaginateRequest) -> Self {
        Self {
            page: pr.page.unwrap_or(0),
            page_size: pr.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        }
    }
}

impl Into<pb::PaginateRequest> for PaginateRequest {
    fn into(self) -> pb::PaginateRequest {
        pb::PaginateRequest {
            page: Some(self.page),
            page_size: Some(self.page_size),
        }
    }
}
