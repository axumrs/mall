use crate::model::{self, U64};

/// 设置商品属性
pub async fn set<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &model::GoodsAttr,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query(r#"INSERT INTO goods_attrs (goods_id, sku, ver) VALUES($1,$2,$3) ON CONFLICT (goods_id) DO UPDATE SET sku=$2, ver=EXCLUDED.ver+1"#)
        .bind(&m.goods_id)
        .bind(&m.sku)
        .bind(&m.ver)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 查找商品属性
pub async fn find<'a>(
    e: impl sqlx::PgExecutor<'a>,
) -> Result<Option<model::GoodsAttr>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT goods_id, sku, ver FROM goods_attrs WHERE 1=1");

    q.build_query_as().fetch_optional(e).await
}

/// 更新库存
pub async fn update_sock<'a>(
    e: impl sqlx::PgExecutor<'a>,
    goods_id: &str,
    sku_key: &str,
    ver: U64,
    increment: i32,
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("UPDATE goods_attrs SET ");

    q.push("sku['data'][")
        .push_bind(sku_key)
        .push("]['stock'] = to_jsonb((sku['data'][")
        .push_bind(sku_key)
        .push("]['stock'])::INTEGER + ")
        .push_bind(increment)
        .push(")");

    q.push(", ver = ").push_bind(ver + U64::from(1u64));

    q.push(" WHERE goods_id=")
        .push_bind(goods_id)
        .push(" AND ver=")
        .push_bind(ver);

    println!("{}", q.sql());

    let aff = q.build().execute(e).await?.rows_affected();
    Ok(aff)
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, vec};

    use crate::model::{self, U32, U64};
    const TEST_GOODS_ID: &str = "ckmivtsdrfagh6slh3tg";

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_db_create_goods_attr() {
        let conn = get_conn().await;
        let meta = model::GoodsSKUMeta {
            names: vec!["颜色", "尺寸", "风格"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            items: vec![
                vec!["红色", "蓝色", "米色"]
                    .iter()
                    .map(|v| v.to_string())
                    .collect(),
                vec!["S", "M", "XL"].iter().map(|v| v.to_string()).collect(),
                vec!["圆领", "V领"].iter().map(|v| v.to_string()).collect(),
            ],
        };

        let meta_with_key = meta.items_with_key();

        let mut sku_data = HashMap::new();
        let mut i = 0;
        for (key, items) in meta_with_key.iter().cloned() {
            let items_str = key.clone();
            sku_data.insert(
                key,
                model::GoodsSKUData {
                    items,
                    items_str,
                    stock: U32::from(30 + i),
                    price: U32::from_yuan(90 + i),
                    origin_price: U32::from_yuan(100 + i),
                    sort: i,
                },
            );
            i += 1;
        }

        let sku = model::GoodsSKU {
            meta,
            data: sku_data,
        };
        let ga = model::GoodsAttr {
            goods_id: TEST_GOODS_ID.to_string(),
            sku: sqlx::types::Json::from(sku),
            ver: U64::from(0 as u64),
        };
        let aff = super::set(&conn, &ga).await.unwrap();
        println!("{}", aff);
    }

    #[tokio::test]
    async fn test_db_find_goods_attr() {
        let conn = get_conn().await;
        let ga = super::find(&conn).await.unwrap();
        println!("{:?}", ga);
    }

    #[tokio::test]
    async fn test_db_update_goods_stock() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        let ga = match super::find(&mut *tx).await {
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("Failed to update {:?}", err);
            }
            Ok(ga) => ga,
        };
        let ga = ga.unwrap();

        let aff = match super::update_sock(&mut *tx, TEST_GOODS_ID, "米色-M-V领", ga.ver, -1).await
        {
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("Failed to update {:?}", err);
            }
            Ok(aff) => aff,
        };

        tx.commit().await.unwrap();
        println!("{:?}", aff);
    }
}
