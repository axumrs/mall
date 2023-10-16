use serde::{Deserialize, Serialize};

use crate::pb;

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
