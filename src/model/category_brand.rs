use serde::{Deserialize, Serialize};

use crate::pb;

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
    pub brand_id: String,
    pub brand_name: String,
    pub brand_logo: String,
    pub brand_is_del: bool,
    pub brand_dateline: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CategoryWithBrands {
    pub category: super::Category,
    pub brands: Vec<super::Brand>,
}

impl From<Vec<CategoryBrandView>> for CategoryWithBrands {
    fn from(vs: Vec<CategoryBrandView>) -> Self {
        if vs.is_empty() {
            return Self::default();
        }
        let mut brands = Vec::with_capacity(vs.len());
        let category = super::Category {
            id: vs[0].id.clone(),
            name: vs[0].name.clone(),
            parent: vs[0].parent.clone(),
            path: vs[0].path.clone(),
            level: vs[0].level.clone(),
            dateline: vs[0].dateline.clone(),
            is_del: vs[0].is_del,
        };
        for item in vs {
            let brand = super::Brand {
                id: item.brand_id,
                name: item.brand_name,
                logo: item.brand_logo,
                is_del: item.brand_is_del,
                dateline: item.brand_dateline,
            };
            brands.push(brand);
        }
        Self { category, brands }
    }
}

impl From<pb::CategoryWithBrands> for CategoryWithBrands {
    fn from(cb: pb::CategoryWithBrands) -> Self {
        let mut brands = Vec::with_capacity(cb.brands.len());
        for brand in cb.brands {
            brands.push(super::Brand::from(brand));
        }
        Self {
            category: cb.category.unwrap_or_default().into(),
            brands,
        }
    }
}

impl Into<pb::CategoryWithBrands> for CategoryWithBrands {
    fn into(self) -> pb::CategoryWithBrands {
        let mut brands = Vec::with_capacity(self.brands.len());
        for brand in self.brands {
            brands.push(brand.into());
        }
        pb::CategoryWithBrands {
            category: Some(self.category.into()),
            brands,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BrandWithCategoies {
    pub brand: super::Brand,
    pub categoies: Vec<super::Category>,
}

impl From<Vec<CategoryBrandView>> for BrandWithCategoies {
    fn from(vs: Vec<CategoryBrandView>) -> Self {
        if vs.is_empty() {
            return Self::default();
        }
        let mut categoies = Vec::with_capacity(vs.len());
        let brand = super::Brand {
            id: vs[0].brand_id.clone(),
            name: vs[0].brand_name.clone(),
            logo: vs[0].brand_logo.clone(),
            is_del: vs[0].brand_is_del,
            dateline: vs[0].brand_dateline.clone(),
        };

        for item in vs {
            let cate = super::Category {
                id: item.id,
                name: item.name,
                parent: item.parent,
                path: item.path,
                level: item.level,
                dateline: item.dateline,
                is_del: item.is_del,
            };
            categoies.push(cate);
        }

        Self { brand, categoies }
    }
}

impl From<pb::BrandWithCategoies> for BrandWithCategoies {
    fn from(bc: pb::BrandWithCategoies) -> Self {
        let brand = super::Brand::from(bc.brand.unwrap_or_default());
        let mut categoies = Vec::with_capacity(bc.categoies.len());

        for cate in bc.categoies {
            categoies.push(cate.into())
        }

        Self { brand, categoies }
    }
}

impl Into<pb::BrandWithCategoies> for BrandWithCategoies {
    fn into(self) -> pb::BrandWithCategoies {
        let brand = Some(self.brand.into());
        let mut categoies = Vec::with_capacity(self.categoies.len());

        for cate in self.categoies {
            categoies.push(cate.into());
        }
        pb::BrandWithCategoies { brand, categoies }
    }
}
