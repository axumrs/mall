use crate::model;

/// 设置品牌分类
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

/// 清空品牌的分类
pub async fn clear<'a, E>(
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
        let cate_id = match category::create(
            &mut *tx,
            &model::Category {
                name: format!("一级分类-品牌关联"),
                parent: String::from(""),
                dateline: chrono::Local::now(),
                ..Default::default()
            },
        )
        .await
        {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("failed to create category: {:?}", e);
            }
        };
        let brand_ids = vec![
            "ckkfkpsdrfak8jh3sveg",
            "ckkfkpsdrfak8jh3svf0",
            // "ckkfkpsdrfak8jh3svfg",
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
}
