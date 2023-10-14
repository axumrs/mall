use crate::model;

/// 设置品牌分类
///
/// **通常需要先调用 `clear()` 函数清空已设置的品牌**
pub async fn set<'a>(
    e: impl sqlx::PgExecutor<'a>,
    category_id: &'a str,
    brand_ids: &'a [&'a str],
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("INSERT INTO category_brands (brand_id,category_id) ");

    let args = brand_ids.iter().map(|&bid| model::CategoryBrand {
        brand_id: bid.to_string(),
        category_id: category_id.to_string(),
    });

    q.push_values(args, |mut b, cb| {
        b.push_bind(cb.brand_id).push_bind(cb.category_id);
    });

    let r = q.build().execute(e).await?;
    return Ok(r.rows_affected());
}

pub async fn list_with_brands<'a>(
    e: impl sqlx::PgExecutor<'a>,
) -> Result<Vec<model::CategoryWithBrands>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, "name", parent, "path", "level", dateline, is_del, brand_ids, brand_names, brand_logos, brand_is_dels, brand_datelines, brand_names_str FROM v_category_with_brands ORDER BY id DESC"#,
    );
    q.build_query_as().fetch_all(e).await
}

pub async fn list_with_categoies<'a>(
    e: impl sqlx::PgExecutor<'a>,
) -> Result<Vec<model::BrandWithCategoies>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT brand_id, brand_name, brand_logo, brand_is_del, brand_dateline, ids, names, names_str, parents, levels, paths, datelines, is_dels FROM v_brand_with_categoies"#,
    );
    q.build_query_as().fetch_all(e).await
}

/// 清空品牌的分类
pub async fn clear<'a>(
    e: impl sqlx::PgExecutor<'a>,
    category_id: &'a str,
) -> Result<u64, sqlx::Error> {
    let r = sqlx::query("DELETE FROM category_brands WHERE category_id = $1")
        .bind(category_id)
        .execute(e)
        .await?;
    return Ok(r.rows_affected());
}

#[cfg(test)]
mod test {
    use crate::{db::category, model};

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_db_set_category_brands() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        // let cate_id = match category::create(
        //     &mut *tx,
        //     &model::Category {
        //         name: format!("一级分类-品牌关联1"),
        //         parent: String::from(""),
        //         dateline: chrono::Local::now(),
        //         ..Default::default()
        //     },
        // )
        // .await
        // {
        //     Ok(id) => id,
        //     Err(e) => {
        //         tx.rollback().await.unwrap();
        //         panic!("failed to create category: {:?}", e);
        //     }
        // };
        let cate_id = "cji1llcdrfap1bhmp76g".to_string();
        let brand_ids = vec![
            "ckkfkpsdrfak8jh3sveg",
            "ckkfkpsdrfak8jh3svf0",
            "ckkfkpsdrfak8jh3svfg",
            // "ckkfkpsdrfak8jh3svg0",
            // "ckkfkpsdrfak8jh3svgg",
            "ckkfkpsdrfak8jh3svh0",
            "ckkfkpsdrfak8jh3svhg",
        ];
        let r = match super::set(&mut *tx, &cate_id, &brand_ids).await {
            Ok(r) => r,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("failed to set category's brand(s): {:?}", e);
            }
        };
        tx.commit().await.unwrap();
        assert!(r > 0);
    }

    #[tokio::test]
    async fn test_db_clear_category_brands() {
        let conn = get_conn().await;
        let category_id = "ckkfod4drfam60t44b5g";
        let r = super::clear(&conn, category_id).await.unwrap();
        assert!(r > 0);
    }

    #[tokio::test]
    async fn test_db_list_with_brands() {
        let conn = get_conn().await;
        let cbs = super::list_with_brands(&conn).await.unwrap();
        for cb in cbs.iter() {
            println!("{:?} , {}", cb.has_brands(), cb.brands_len());
        }
    }

    #[tokio::test]
    async fn test_db_list_with_categoies() {
        let conn = get_conn().await;
        let bcs = super::list_with_categoies(&conn).await.unwrap();
        for bc in bcs.iter() {
            println!("{:?}", bc.levels());
        }
    }
}
