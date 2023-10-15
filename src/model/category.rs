use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "category_level")]
// #[repr(i32)]
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

/// 分类树。对应数据库 `v_tree_pure` 视图
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct TreePure {
    #[sqlx(flatten)]
    pub category: Category,
    pub fullname: String,
}

// ---- 查找分类 ----

pub enum FindCategoryBy {
    ID(String),
    NameAndParent(CategoryNameAndParentRequest),
}

pub struct FindCategoryRequest {
    pub by: FindCategoryBy,
    pub level: Option<CategoryLevel>,
    pub is_del: Option<bool>,
}

impl From<pb::FindCategoryRequest> for FindCategoryRequest {
    fn from(r: pb::FindCategoryRequest) -> Self {
        let level = if let Some(level) = r.level {
            let lv: CategoryLevel = pb::CategoryLevel::from_i32(level)
                .unwrap_or_default()
                .into();
            Some(lv)
        } else {
            None
        };
        let by = match r.by.unwrap() {
            pb::find_category_request::By::Id(id) => FindCategoryBy::ID(id),
            pb::find_category_request::By::NameAndParent(nap) => {
                FindCategoryBy::NameAndParent(nap.into())
            }
        };
        Self {
            by,
            level,
            is_del: r.is_del,
        }
    }
}

impl Into<pb::FindCategoryRequest> for FindCategoryRequest {
    fn into(self) -> pb::FindCategoryRequest {
        let level = if let Some(level) = self.level {
            let lv: pb::CategoryLevel = level.into();
            let lv: i32 = lv.into();
            Some(lv)
        } else {
            None
        };
        let by = match self.by {
            FindCategoryBy::ID(id) => pb::find_category_request::By::Id(id),
            FindCategoryBy::NameAndParent(nap) => {
                pb::find_category_request::By::NameAndParent(nap.into())
            }
        };
        let by = Some(by);
        pb::FindCategoryRequest {
            is_del: self.is_del,
            level,
            by,
        }
    }
}

// --- 分类列表 ---
pub struct ListCategoryRequest {
    pub paginate: db::PaginateRequest,
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub parent: Option<String>,
    pub level: Option<CategoryLevel>,
}

impl From<pb::ListCategoryRequest> for ListCategoryRequest {
    fn from(r: pb::ListCategoryRequest) -> Self {
        let level = if let Some(level) = r.level {
            let lv: CategoryLevel = pb::CategoryLevel::from_i32(level)
                .unwrap_or_default()
                .into();
            Some(lv)
        } else {
            None
        };
        Self {
            paginate: r.paginate.unwrap().into(),
            name: r.name,
            is_del: r.is_del,
            parent: r.parent,
            level,
        }
    }
}
// --- 分类树 ---
pub enum CategoryTreeBy {
    Parent(String),
    Path(String),
}

pub struct CategoryTreeRequest {
    pub by: CategoryTreeBy,
    pub is_del: Option<bool>,
    pub level: Option<CategoryLevel>,
    pub name: Option<String>,
}

impl From<pb::CategoryTreeRequest> for CategoryTreeRequest {
    fn from(r: pb::CategoryTreeRequest) -> Self {
        let level = if let Some(level) = r.level {
            let lv: CategoryLevel = pb::CategoryLevel::from_i32(level).unwrap().into();
            Some(lv)
        } else {
            None
        };

        let by = match r.by.unwrap() {
            pb::category_tree_request::By::Parent(parent) => CategoryTreeBy::Parent(parent),
            pb::category_tree_request::By::Path(path) => CategoryTreeBy::Path(path),
        };

        Self {
            by,
            is_del: r.is_del,
            level,
            name: r.name,
        }
    }
}

impl Into<pb::CategoryTreeRequest> for CategoryTreeRequest {
    fn into(self) -> pb::CategoryTreeRequest {
        let by = match self.by {
            CategoryTreeBy::Parent(parent) => pb::category_tree_request::By::Parent(parent),
            CategoryTreeBy::Path(path) => pb::category_tree_request::By::Path(path),
        };
        let by = Some(by);

        let level = if let Some(level) = self.level {
            let lv: pb::CategoryLevel = level.into();
            Some(lv.into())
        } else {
            None
        };

        pb::CategoryTreeRequest {
            is_del: self.is_del,
            level,
            name: self.name,
            by,
        }
    }
}
