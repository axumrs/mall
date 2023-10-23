use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{pb, utils::Cartesian};

use super::{U32, U64};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GoodsSKUMeta {
    pub names: Vec<String>,
    pub items: Vec<Vec<String>>,
}

impl GoodsSKUMeta {
    /// 生成SKU项的列表
    pub fn table(&self) -> Vec<Vec<String>> {
        let c = Cartesian::new(&self.items);
        c.product()
    }
    /// 生成带KEY的SKU项的列表
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
    /// 生成SKU项的KEY
    pub fn key(item: &Vec<String>) -> String {
        item.join("-")
    }
}

impl From<pb::goods_sku::Meta> for GoodsSKUMeta {
    fn from(m: pb::goods_sku::Meta) -> Self {
        let items = m.items.iter().map(|i| i.items.clone()).collect();
        Self {
            names: m.names,
            items,
        }
    }
}

impl Into<pb::goods_sku::Meta> for GoodsSKUMeta {
    fn into(self) -> pb::goods_sku::Meta {
        pb::goods_sku::Meta {
            names: self.names,
            items: self
                .items
                .iter()
                .map(|i| pb::goods_sku::MetaItems { items: i.clone() })
                .collect(),
        }
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

impl From<pb::goods_sku::DataItem> for GoodsSKUData {
    fn from(d: pb::goods_sku::DataItem) -> Self {
        Self {
            items: d.items,
            items_str: d.items_str,
            stock: U32::from(d.stock),
            price: U32::from(d.price),
            origin_price: U32::from(d.origin_price),
            sort: d.sort,
        }
    }
}

impl Into<pb::goods_sku::DataItem> for GoodsSKUData {
    fn into(self) -> pb::goods_sku::DataItem {
        pb::goods_sku::DataItem {
            items: self.items,
            items_str: self.items_str,
            stock: self.stock.unsigned(),
            price: self.price.unsigned(),
            origin_price: self.origin_price.unsigned(),
            sort: self.sort,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GoodsSKU {
    pub meta: GoodsSKUMeta,
    pub data: HashMap<String, GoodsSKUData>,
}

impl From<pb::GoodsSku> for GoodsSKU {
    fn from(s: pb::GoodsSku) -> Self {
        let mut data: HashMap<String, GoodsSKUData> = HashMap::with_capacity(s.data.len());
        for (k, v) in s.data {
            data.insert(k, v.into());
        }
        Self {
            meta: s.meta.unwrap().into(),
            data,
        }
    }
}

impl Into<pb::GoodsSku> for GoodsSKU {
    fn into(self) -> pb::GoodsSku {
        let mut data = HashMap::with_capacity(self.data.len());
        for (k, v) in self.data {
            data.insert(k, v.into());
        }
        pb::GoodsSku {
            meta: Some(self.meta.into()),
            data,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct GoodsAttr {
    pub goods_id: String,
    pub sku: sqlx::types::Json<GoodsSKU>,
    pub ver: U64,
}

impl From<pb::GoodsAttr> for GoodsAttr {
    fn from(ga: pb::GoodsAttr) -> Self {
        let sku: GoodsSKU = ga.sku.unwrap().into();
        Self {
            goods_id: ga.goods_id,
            sku: sqlx::types::Json::from(sku),
            ver: U64::from(ga.ver),
        }
    }
}

impl Into<pb::GoodsAttr> for GoodsAttr {
    fn into(self) -> pb::GoodsAttr {
        let sku: pb::GoodsSku = self.sku.0.into();
        pb::GoodsAttr {
            goods_id: self.goods_id,
            sku: Some(sku),
            ver: self.ver.unsigned(),
        }
    }
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
