use serde::{Deserialize, Serialize};

use crate::{pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "category_level")]
#[repr(i32)]
pub enum CategoryLevel {
    /// 未指定
    #[default]
    Unspecified = 0,
    /// 一级分类
    Level1 = 1,
    /// 二级分类
    Level2 = 2,
    /// 三级分类
    Level3 = 3,
}

impl std::str::FromStr for CategoryLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "Level1" => Self::Level1,
            "Level2" => Self::Level2,
            "Level3" => Self::Level3,
            _ => Self::Unspecified,
        };
        Ok(r)
    }
}

impl From<pb::CategoryLevel> for CategoryLevel {
    fn from(cl: pb::CategoryLevel) -> Self {
        match cl {
            pb::CategoryLevel::Level1 => Self::Level1,
            pb::CategoryLevel::Level2 => Self::Level2,
            pb::CategoryLevel::Level3 => Self::Level3,
            _ => Self::Unspecified,
        }
    }
}

impl Into<pb::CategoryLevel> for CategoryLevel {
    fn into(self) -> pb::CategoryLevel {
        match self {
            Self::Level1 => pb::CategoryLevel::Level1,
            Self::Level2 => pb::CategoryLevel::Level2,
            Self::Level3 => pb::CategoryLevel::Level3,
            _ => pb::CategoryLevel::Unspecified,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub path: String,
    pub level: CategoryLevel,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

impl From<pb::Category> for Category {
    fn from(c: pb::Category) -> Self {
        let level = CategoryLevel::from(c.level());
        Self {
            id: c.id,
            name: c.name,
            parent: c.parent,
            path: c.path,
            level,
            dateline: dt::prost2chrono(&c.dateline),
            is_del: c.is_del,
        }
    }
}

impl Into<pb::Category> for Category {
    fn into(self) -> pb::Category {
        let level: pb::CategoryLevel = self.level.into();
        let level = level.into();
        pb::Category {
            id: self.id,
            name: self.name,
            parent: self.parent,
            path: self.path,
            level,
            dateline: dt::chrono2prost(&self.dateline),
            is_del: self.is_del,
        }
    }
}

pub struct CategoryNameAndParentRequest {
    pub name: String,
    pub parent: String,
}

impl CategoryNameAndParentRequest {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl From<pb::CategoryNameAndParentRequest> for CategoryNameAndParentRequest {
    fn from(r: pb::CategoryNameAndParentRequest) -> Self {
        Self {
            name: r.name,
            parent: r.parent,
        }
    }
}

impl Into<pb::CategoryNameAndParentRequest> for CategoryNameAndParentRequest {
    fn into(self) -> pb::CategoryNameAndParentRequest {
        pb::CategoryNameAndParentRequest {
            name: self.name,
            parent: self.parent,
        }
    }
}

pub struct CategoryExistsRequest {
    pub name_and_parent: CategoryNameAndParentRequest,
    pub id: Option<String>,
}

impl From<pb::CategoryExistsRequest> for CategoryExistsRequest {
    fn from(r: pb::CategoryExistsRequest) -> Self {
        Self {
            name_and_parent: r.name_and_parent.unwrap_or_default().into(),
            id: r.id,
        }
    }
}

impl Into<pb::CategoryExistsRequest> for CategoryExistsRequest {
    fn into(self) -> pb::CategoryExistsRequest {
        pb::CategoryExistsRequest {
            name_and_parent: Some(self.name_and_parent.into()),
            id: self.id,
        }
    }
}
