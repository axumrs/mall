use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

use super::{BrandFindBy, Category, CategoryLevel, FindCategoryBy};

/// 分类与品牌的关系。对应数据库 `category_brands` 表
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct CategoryBrand {
    pub brand_id: String,
    pub category_id: String,
}

/// 分类与品牌。对应数据库 `v_category_brands` 视图
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct CategoryBrandView {
    // 分类
    pub id: String,
    pub name: String,
    pub parent: String,
    pub path: String,
    pub level: super::CategoryLevel,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
    // 品牌
    pub brand_id: Option<String>,
    pub brand_name: Option<String>,
    pub brand_logo: Option<String>,
    pub brand_is_del: Option<bool>,
    pub brand_dateline: Option<chrono::DateTime<chrono::Local>>,
}

/// 分类的品牌信息。对应数据库 `v_category_with_brands` 视图
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct CategoryWithBrands {
    // 分类
    pub id: String,
    pub name: String,
    pub parent: String,
    pub path: String,
    pub level: super::CategoryLevel,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
    // 品牌
    pub brand_ids: Vec<Option<String>>,
    pub brand_names: Vec<Option<String>>,
    pub brand_logos: Vec<Option<String>>,
    pub brand_is_dels: Vec<Option<bool>>,
    pub brand_datelines: Vec<Option<chrono::DateTime<chrono::Local>>>,
    pub brand_names_str: String,
}

impl CategoryWithBrands {
    pub fn has_brands(&self) -> bool {
        (!self.brand_ids.is_empty()) && self.brand_ids.iter().all(|id| id.is_some())
    }
    pub fn brands_len(&self) -> usize {
        if !self.has_brands() {
            0
        } else {
            self.brand_ids.len()
        }
    }
}

impl From<pb::CategoryWithBrands> for CategoryWithBrands {
    fn from(pcb: pb::CategoryWithBrands) -> Self {
        let cate = pcb.category.unwrap();
        let mut brand_ids = Vec::with_capacity(pcb.brands.len());
        let mut brand_names = Vec::with_capacity(pcb.brands.len());
        let mut brand_logos = Vec::with_capacity(pcb.brands.len());
        let mut brand_is_dels = Vec::with_capacity(pcb.brands.len());
        let mut brand_datelines = Vec::with_capacity(pcb.brands.len());
        let mut brand_names_str = Vec::with_capacity(pcb.brands.len());

        for b in pcb.brands.iter() {
            brand_ids.push(Some(b.id.clone()));
            brand_names.push(Some(b.name.clone()));
            brand_logos.push(Some(b.logo.clone()));
            brand_is_dels.push(Some(b.is_del));
            brand_datelines.push(Some(dt::prost2chrono(&b.dateline)));
            brand_names_str.push(b.name.clone());
        }

        let brand_names_str = format!(",{},", brand_names_str.join(","));

        let level = super::CategoryLevel::from(cate.level());

        Self {
            id: cate.id,
            name: cate.name,
            parent: cate.parent,
            path: cate.path,
            level,
            dateline: dt::prost2chrono(&cate.dateline),
            is_del: cate.is_del,
            brand_ids,
            brand_names,
            brand_logos,
            brand_is_dels,
            brand_datelines,
            brand_names_str,
        }
    }
}

impl Into<pb::CategoryWithBrands> for CategoryWithBrands {
    fn into(self) -> pb::CategoryWithBrands {
        let dateline = dt::chrono2prost(&self.dateline);
        let level: pb::CategoryLevel = self.level.into();
        let mut brands = Vec::with_capacity(self.brands_len());
        if self.has_brands() {
            for i in 0..self.brands_len() {
                let b = pb::Brand {
                    id: self.brand_ids[i].clone().unwrap(),
                    name: self.brand_names[i].clone().unwrap(),
                    logo: self.brand_logos[i].clone().unwrap(),
                    is_del: self.brand_is_dels[i].clone().unwrap(),
                    dateline: dt::chrono2prost(&self.brand_datelines[i].clone().unwrap()),
                };
                brands.push(b);
            }
        }

        let cate = pb::Category {
            id: self.id,
            name: self.name,
            parent: self.parent,
            path: self.path,
            level: level.into(),
            dateline,
            is_del: self.is_del,
        };

        pb::CategoryWithBrands {
            category: Some(cate),
            brands,
        }
    }
}

/// 品牌的分类信息。对应数据库 `v_brand_with_categoies` 视图
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct BrandWithCategoies {
    // 品牌
    pub brand_id: String,
    pub brand_name: String,
    pub brand_logo: String,
    pub brand_is_del: bool,
    pub brand_dateline: chrono::DateTime<chrono::Local>,

    // 分类
    pub ids: Vec<Option<String>>,
    pub names: Vec<Option<String>>,
    pub parents: Vec<Option<String>>,
    pub paths: Vec<Option<String>>,
    pub levels: Vec<Option<String>>,
    pub datelines: Vec<Option<chrono::DateTime<chrono::Local>>>,
    pub is_dels: Vec<Option<bool>>,
    pub names_str: String,
}

impl BrandWithCategoies {
    pub fn has_categoies(&self) -> bool {
        (!self.ids.is_empty()) && self.ids.iter().all(|id| id.is_some())
    }
    pub fn categoies_len(&self) -> usize {
        if !self.has_categoies() {
            0
        } else {
            self.ids.len()
        }
    }
    pub fn levels(&self) -> Vec<super::CategoryLevel> {
        if !self.has_categoies() {
            return vec![];
        }
        let mut ls = Vec::with_capacity(self.categoies_len());
        for level in self.levels.iter() {
            let l = super::CategoryLevel::from_str(level.clone().unwrap().as_str()).unwrap();
            ls.push(l);
        }
        ls
    }
}

impl From<pb::BrandWithCategoies> for BrandWithCategoies {
    fn from(pbc: pb::BrandWithCategoies) -> Self {
        let b = pbc.brand.unwrap();
        let mut ids = Vec::with_capacity(pbc.categoies.len());
        let mut names = Vec::with_capacity(pbc.categoies.len());
        let mut parents = Vec::with_capacity(pbc.categoies.len());
        let mut paths = Vec::with_capacity(pbc.categoies.len());
        let mut levels = Vec::with_capacity(pbc.categoies.len());
        let mut datelines = Vec::with_capacity(pbc.categoies.len());
        let mut is_dels = Vec::with_capacity(pbc.categoies.len());
        let mut names_str = Vec::with_capacity(pbc.categoies.len());

        for c in pbc.categoies.iter() {
            ids.push(Some(c.id.clone()));
            names.push(Some(c.name.clone()));
            parents.push(Some(c.parent.clone()));
            paths.push(Some(c.path.clone()));
            levels.push(Some(format!("{:?}", super::CategoryLevel::from(c.level()))));
            datelines.push(Some(dt::prost2chrono(&c.dateline)));
            is_dels.push(Some(c.is_del));
            names_str.push(c.name.clone());
        }

        let names_str = format!(",{},", names_str.join(","));

        Self {
            brand_id: b.id,
            brand_name: b.name,
            brand_logo: b.logo,
            brand_is_del: b.is_del,
            brand_dateline: dt::prost2chrono(&b.dateline),
            ids,
            names,
            parents,
            paths,
            levels,
            datelines,
            is_dels,
            names_str,
        }
    }
}

impl Into<pb::BrandWithCategoies> for BrandWithCategoies {
    fn into(self) -> pb::BrandWithCategoies {
        let brand = pb::Brand {
            id: self.brand_id.clone(),
            name: self.brand_name.clone(),
            logo: self.brand_logo.clone(),
            is_del: self.brand_is_del,
            dateline: dt::chrono2prost(&self.brand_dateline),
        };

        let mut categoies = Vec::with_capacity(self.categoies_len());

        if self.has_categoies() {
            for i in 0..self.categoies_len() {
                let c = pb::Category {
                    id: self.ids[i].clone().unwrap(),
                    name: self.names[i].clone().unwrap(),
                    parent: self.parents[i].clone().unwrap(),
                    path: self.paths[i].clone().unwrap(),
                    level: self.levels[i].clone().unwrap().parse().unwrap(),
                    dateline: dt::chrono2prost(&self.datelines[i].clone().unwrap()),
                    is_del: self.is_dels[i].clone().unwrap(),
                };
                categoies.push(c);
            }
        }

        pb::BrandWithCategoies {
            brand: Some(brand),
            categoies,
        }
    }
}

/// 分类树，带品牌信息。对应数据库 `v_tree` 视图
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Tree {
    #[sqlx(flatten)]
    pub category_with_brands: CategoryWithBrands,
    pub fullname: String,
}

impl From<pb::CategoryWithBrandsTree> for Tree {
    fn from(t: pb::CategoryWithBrandsTree) -> Self {
        Self {
            category_with_brands: t.category_with_brands.unwrap().into(),
            fullname: t.fullname,
        }
    }
}

impl Into<pb::CategoryWithBrandsTree> for Tree {
    fn into(self) -> pb::CategoryWithBrandsTree {
        pb::CategoryWithBrandsTree {
            category_with_brands: Some(self.category_with_brands.into()),
            fullname: self.fullname,
        }
    }
}

// --- 查找分类 ---
pub struct FindCategoryWithBrandsRequest {
    pub by: FindCategoryBy,
    pub is_del: Option<bool>,
    pub level: Option<CategoryLevel>,
    pub brand_name: Option<String>,
}

impl From<pb::FindCategoryWithBrandsRequest> for FindCategoryWithBrandsRequest {
    fn from(r: pb::FindCategoryWithBrandsRequest) -> Self {
        let level = if let Some(level) = r.level {
            Some(pb::CategoryLevel::from_i32(level).unwrap().into())
        } else {
            None
        };

        let by = match r.by.unwrap() {
            pb::find_category_with_brands_request::By::Id(id) => FindCategoryBy::ID(id),
            pb::find_category_with_brands_request::By::NameAndParent(nap) => {
                FindCategoryBy::NameAndParent(nap.into())
            }
        };
        Self {
            by,
            is_del: r.is_del,
            level,
            brand_name: r.brand_name,
        }
    }
}

impl Into<pb::FindCategoryWithBrandsRequest> for FindCategoryWithBrandsRequest {
    fn into(self) -> pb::FindCategoryWithBrandsRequest {
        let level = if let Some(level) = self.level {
            Some(pb::CategoryLevel::from_i32(level as i32).unwrap().into())
        } else {
            None
        };

        let by = match self.by {
            FindCategoryBy::ID(id) => pb::find_category_with_brands_request::By::Id(id),
            FindCategoryBy::NameAndParent(nap) => {
                pb::find_category_with_brands_request::By::NameAndParent(nap.into())
            }
        };
        let by = Some(by);

        pb::FindCategoryWithBrandsRequest {
            is_del: self.is_del,
            level,
            brand_name: self.brand_name,
            by,
        }
    }
}

// --- 分类列表 ---
pub struct ListCategoryWithBrandsRequest {
    pub paginate: db::PaginateRequest,
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub parent: Option<String>,
    pub level: Option<CategoryLevel>,
    pub brand_name: Option<String>,
}

impl From<pb::ListCategoryWithBrandsRequest> for ListCategoryWithBrandsRequest {
    fn from(r: pb::ListCategoryWithBrandsRequest) -> Self {
        let level = if let Some(level) = r.level {
            Some(pb::CategoryLevel::from_i32(level).unwrap().into())
        } else {
            None
        };
        Self {
            paginate: r.paginate.unwrap().into(),
            name: r.name,
            is_del: r.is_del,
            parent: r.parent,
            level,
            brand_name: r.brand_name,
        }
    }
}

impl Into<pb::ListCategoryWithBrandsRequest> for ListCategoryWithBrandsRequest {
    fn into(self) -> pb::ListCategoryWithBrandsRequest {
        let level = if let Some(level) = self.level {
            let level: pb::CategoryLevel = level.into();
            Some(level.into())
        } else {
            None
        };

        pb::ListCategoryWithBrandsRequest {
            paginate: Some(self.paginate.into()),
            name: self.name,
            is_del: self.is_del,
            parent: self.parent,
            level,
            brand_name: self.brand_name,
        }
    }
}

// --- 查找品牌 ---

pub struct FindBrandWithCategoriesRequest {
    pub by: BrandFindBy,
    pub is_del: Option<bool>,
    pub category_name: Option<String>,
}

impl From<pb::FindBrandWithCategoiesRequest> for FindBrandWithCategoriesRequest {
    fn from(r: pb::FindBrandWithCategoiesRequest) -> Self {
        let by = match r.by.unwrap() {
            pb::find_brand_with_categoies_request::By::Id(id) => BrandFindBy::ID(id),
            pb::find_brand_with_categoies_request::By::Name(name) => BrandFindBy::Name(name),
        };
        Self {
            by,
            is_del: r.is_del,
            category_name: r.category_name,
        }
    }
}

impl Into<pb::FindBrandWithCategoiesRequest> for FindBrandWithCategoriesRequest {
    fn into(self) -> pb::FindBrandWithCategoiesRequest {
        let by = match self.by {
            BrandFindBy::ID(id) => pb::find_brand_with_categoies_request::By::Id(id),
            BrandFindBy::Name(name) => pb::find_brand_with_categoies_request::By::Name(name),
        };
        let by = Some(by);

        pb::FindBrandWithCategoiesRequest {
            is_del: self.is_del,
            category_name: self.category_name,
            by,
        }
    }
}

// --- 品牌列表 ---
pub struct ListBrandWithCategoriesRequest {
    pub paginate: db::PaginateRequest,
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub category_name: Option<String>,
}

impl From<pb::ListBrandWithCategoiesRequest> for ListBrandWithCategoriesRequest {
    fn from(r: pb::ListBrandWithCategoiesRequest) -> Self {
        Self {
            paginate: r.paginate.unwrap().into(),
            name: r.name,
            is_del: r.is_del,
            category_name: r.category_name,
        }
    }
}

impl Into<pb::ListBrandWithCategoiesRequest> for ListBrandWithCategoriesRequest {
    fn into(self) -> pb::ListBrandWithCategoiesRequest {
        pb::ListBrandWithCategoiesRequest {
            paginate: Some(self.paginate.into()),
            name: self.name,
            is_del: self.is_del,
            category_name: self.category_name,
        }
    }
}

// --- 设置分类-品牌 ---
pub struct SetCategoryBrandRequest {
    pub category_id: String,
    pub brand_ids: Vec<String>,
}

impl From<pb::SetCategoryBrandsRequest> for SetCategoryBrandRequest {
    fn from(r: pb::SetCategoryBrandsRequest) -> Self {
        Self {
            category_id: r.category_id,
            brand_ids: r.brand_ids,
        }
    }
}

impl Into<pb::SetCategoryBrandsRequest> for SetCategoryBrandRequest {
    fn into(self) -> pb::SetCategoryBrandsRequest {
        pb::SetCategoryBrandsRequest {
            category_id: self.category_id,
            brand_ids: self.brand_ids,
        }
    }
}

// --- 清除分类品牌 ---
pub struct ClearCategoryBrandsRequest {
    pub category_id: String,
}

impl From<pb::ClearCategoryBrandsRequest> for ClearCategoryBrandsRequest {
    fn from(r: pb::ClearCategoryBrandsRequest) -> Self {
        Self {
            category_id: r.category_id,
        }
    }
}

impl Into<pb::ClearCategoryBrandsRequest> for ClearCategoryBrandsRequest {
    fn into(self) -> pb::ClearCategoryBrandsRequest {
        pb::ClearCategoryBrandsRequest {
            category_id: self.category_id,
        }
    }
}

// --- 创建带品牌的分类 ---

pub struct CreateCategoryWithBrandsRequest {
    pub category: Category,
    pub brand_ids: Vec<String>,
}

impl From<pb::CreateCategoryWithBrandsRequest> for CreateCategoryWithBrandsRequest {
    fn from(r: pb::CreateCategoryWithBrandsRequest) -> Self {
        Self {
            category: r.category.unwrap().into(),
            brand_ids: r.brand_ids,
        }
    }
}

impl Into<pb::CreateCategoryWithBrandsRequest> for CreateCategoryWithBrandsRequest {
    fn into(self) -> pb::CreateCategoryWithBrandsRequest {
        pb::CreateCategoryWithBrandsRequest {
            category: Some(self.category.into()),
            brand_ids: self.brand_ids,
        }
    }
}
