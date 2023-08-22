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
    let sql = r#"UPDATE categoies SET
	"name" = (SELECT COALESCE ((SELECT LEFT( $1 || '#' || name, 100 )  FROM categoies WHERE name = $2 and id<>$1), $2)),
	parent = $3,
	"path" = (SELECT COALESCE  ((SELECT "path" FROM categoies  WHERE id=$3) , '//')) || $1 || '/',
	"level" = (SELECT CASE COALESCE ((SELECT "level" FROM categoies where id=$3), 'Unspecified'::category_level) WHEN 'Level1'::category_level THEN 'Level2'::category_level WHEN 'Level2'::category_level THEN 'Level3'::category_level ELSE 'Level1'::category_level END)
WHERE id = $1"#;
    let r = sqlx::query(sql)
        .bind(&c.id)
        .bind(&c.name)
        .bind(&c.parent)
        .execute(e)
        .await?;
    Ok(r.rows_affected())
}

/// 获取子分类
pub async fn get_children<'a>(
    e: impl sqlx::PgExecutor<'a>,
    parent_path: &str,
    parent: &str,
) -> Result<Vec<model::Category>, sqlx::Error> {
    let parent_path = format!("{}%/", parent_path);
    sqlx::query_as(r#"SELECT id, "name", parent, "path", "level", dateline, is_del FROM categoies WHERE "path" LIKE $1 AND id<>$2 ORDER BY "level" ASC, id ASC"#).bind(&parent_path).bind(parent).fetch_all(e).await
}

/// 修改名称
pub async fn edit_name<'a>(
    e: impl sqlx::PgExecutor<'a>,
    id: &str,
    name: &str,
) -> Result<u64, sqlx::Error> {
    let r = sqlx::query("UPDATE categoies SET name=$1 WHERE id=$2")
        .bind(name)
        .bind(id)
        .execute(e)
        .await?;

    Ok(r.rows_affected())
}

/// 删除或修改
pub async fn del_or_restore(conn: &sqlx::PgPool, id: String, is_del: bool) -> crate::Result<u64> {
    super::del_or_restore(conn, "categoies", id, is_del).await
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
        let cate: model::Category = sqlx::query_as("SELECT * FROM categoies WHERE id=$1")
            .bind("cji1llcdrfap1bhmp7f0")
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        println!("{:?}", cate);
        let cate = model::Category {
            name: "一级分类2".to_string(),
            ..cate
        };
        if let Err(e) = super::edit(&mut *tx, &cate).await {
            tx.rollback().await.unwrap();
            panic!("{}", e.to_string());
        }
        // 更新子分类

        let children = super::get_children(&mut *tx, &cate.path, &cate.id)
            .await
            .unwrap();
        for c in children.iter() {
            println!("{:?}", c);
            {
                if let Err(e) = super::edit(&mut *tx, c).await {
                    tx.rollback().await.unwrap();
                    panic!("{}", e.to_string());
                }
            }
        }
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_db_edit_name_for_category() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        let cate: model::Category = sqlx::query_as("SELECT * FROM categoies WHERE id=$1")
            .bind("cji1llcdrfap1bhmp7f0")
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        println!("{:?}", cate);

        let cate = model::Category {
            name: "一级分类3".to_string(),
            ..cate
        };

        let exists = super::exists(
            &mut *tx,
            &model::CategoryExistsRequest {
                name_and_parent: model::CategoryNameAndParentRequest {
                    name: cate.name.clone(),
                    parent: cate.parent.clone(),
                },
                id: Some(cate.id.clone()),
            },
        )
        .await
        .unwrap();

        if exists {
            panic!("同名的分类已存在");
        }

        super::edit_name(&mut *tx, &cate.id, &cate.name)
            .await
            .unwrap();

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_db_del_or_restore_category() {
        let conn = get_conn().await;
        let aff = super::del_or_restore(&conn, "cji1llcdrfap1bhmp7f0".to_string(), true)
            .await
            .unwrap();
        assert!(aff > 0);
    }
}
