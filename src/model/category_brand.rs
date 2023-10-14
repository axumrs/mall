use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{pb, utils::dt};

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
