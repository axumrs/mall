use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::Cartesian;

use super::{U32, U64};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GoodsSKUMeta {
    pub names: Vec<String>,
    pub items: Vec<Vec<String>>,
}

impl GoodsSKUMeta {
    pub fn table(&self) -> Vec<Vec<String>> {
        let c = Cartesian::new(&self.items);
        c.product()
    }
    pub fn items_with_key(&self) -> Vec<(String, Vec<String>)> {
        let t = self.table();
        t.iter()
            .cloned()
            .map(|i| {
                let key = Self::key(&i);
                (key, i)
            })
            .collect()
    }
    pub fn key(item: &Vec<String>) -> String {
        item.join("-")
    }
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GoodsSKUData {
    pub items: Vec<String>,
    pub items_str: String,
    pub stock: U32,
    pub price: U32,
    pub origin_price: U32,
    pub sort: i32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GoodsSKU {
    pub meta: GoodsSKUMeta,
    pub data: HashMap<String, GoodsSKUData>,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct GoodsAttr {
    pub goods_id: String,
    pub sku: sqlx::types::Json<GoodsSKU>,
    pub ver: U64,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_goods_sku_meta_table() {
        let sku_items = vec![
            vec!["红色", "蓝色", "米色"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            vec!["S", "M", "XL"].iter().map(|v| v.to_string()).collect(),
            vec!["圆领", "V领"].iter().map(|v| v.to_string()).collect(),
        ];
        let meta = super::GoodsSKUMeta {
            names: vec!["颜色", "尺寸", "风格"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            items: sku_items,
        };
        let table = meta.table();
        println!("TABLE: {:?}", table);
    }
    #[test]
    fn test_goods_sku_meta_with_key_table() {
        let sku_items = vec![
            vec!["红色", "蓝色", "米色"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            vec!["S", "M", "XL"].iter().map(|v| v.to_string()).collect(),
            vec!["圆领", "V领"].iter().map(|v| v.to_string()).collect(),
        ];
        let meta = super::GoodsSKUMeta {
            names: vec!["颜色", "尺寸", "风格"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            items: sku_items,
        };
        let table = meta.items_with_key();
        println!("WITH KEY TABLE: {:?}", table);
    }
}
