use crate::{model, utils};

pub async fn create<'a, E>(e: E, c: &model::Category) -> Result<String, sqlx::Error>
where
    E: sqlx::PgExecutor<'a>,
{
    let id = utils::id::new();
    sqlx::query(r#"INSERT INTO categoies (id, "name", parent, "path", "level", dateline, is_del)
	SELECT $1, $2,$3, (SELECT COALESCE  ((SELECT "path" FROM categoies  WHERE id=$3) , '//')) || $1 || '/', (SELECT CASE COALESCE ((SELECT "level" FROM categoies where id=$3), 'Unspecified'::category_level) WHEN 'Level1'::category_level THEN 'Level2'::category_level WHEN 'Level2'::category_level THEN 'Level3'::category_level ELSE 'Level1'::category_level END),$4,$5"#)
    .bind(&id)
    .bind(&c.name)
    .bind(&c.parent)
    .bind(&c.dateline)
    .bind(&c.is_del)
    .execute(e).await?;
    Ok(id)
}

pub async fn exists<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::CategoryExistsRequest,
) -> Result<bool, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM categoies WHERE 1=1");
    q.push(" AND (name=")
        .push_bind(&r.name_and_parent.name)
        .push(" AND parent=")
        .push_bind(&r.name_and_parent.parent)
        .push(")");
    if let Some(id) = &r.id {
        q.push(" AND id<>").push_bind(id);
    }

    let count: (i64,) = q.build_query_as().fetch_one(e).await?;
    Ok(count.0 > 0)
}

/// 更新分类
pub async fn edit<'a>(
    e: impl sqlx::PgExecutor<'a>,
    c: &model::Category,
) -> Result<u64, sqlx::Error> {
    unimplemented!()
}

/// 级联更新分类
pub async fn edit_cascade<'a>(
    e: impl sqlx::PgExecutor<'a>,
    parent: &str,
) -> Result<u64, sqlx::Error> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use crate::model;

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_db_create_category() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        let cate = model::Category {
            name: format!("一级分类1"),
            parent: String::from(""),
            dateline: chrono::Local::now(),
            ..Default::default()
        };
        let id = match super::create(&mut *tx, &cate).await {
            Ok(id) => id,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{}", err.to_string());
            }
        };
        assert!(!id.is_empty());
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_db_batch_create_category() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        for i in 1..=5 {
            let cate = model::Category {
                name: format!("一级分类{}", i),
                parent: String::from(""),
                dateline: chrono::Local::now(),
                ..Default::default()
            };
            let level1_id = match super::create(&mut *tx, &cate).await {
                Ok(id) => id,
                Err(err) => {
                    tx.rollback().await.unwrap();
                    panic!("{}", err.to_string());
                }
            };
            assert!(!level1_id.is_empty());

            for j in 1..=4 {
                let cate = model::Category {
                    name: format!("二级分类-{}-{}", i, j),
                    parent: level1_id.clone(),
                    dateline: chrono::Local::now(),
                    ..Default::default()
                };
                let level2_id = match super::create(&mut *tx, &cate).await {
                    Ok(id) => id,
                    Err(err) => {
                        tx.rollback().await.unwrap();
                        panic!("{}", err.to_string());
                    }
                };
                assert!(!level2_id.is_empty());

                for k in 1..=3 {
                    let cate = model::Category {
                        name: format!("三级分类-{}-{}-{}", i, j, k),
                        parent: level2_id.clone(),
                        dateline: chrono::Local::now(),
                        ..Default::default()
                    };
                    let level3_id = match super::create(&mut *tx, &cate).await {
                        Ok(id) => id,
                        Err(err) => {
                            tx.rollback().await.unwrap();
                            panic!("{}", err.to_string());
                        }
                    };
                    assert!(!level3_id.is_empty());
                }
            }
        }

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_db_edit_category() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        tx.commit().await.unwrap();
    }
}
