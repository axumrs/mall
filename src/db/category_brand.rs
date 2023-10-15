use crate::{model, Error};

/// 设置品牌分类
///
/// **通常需要先调用 `clear()` 函数清空已设置的品牌**
pub async fn set<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::SetCategoryBrandRequest,
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("INSERT INTO category_brands (brand_id,category_id) ");

    let args = r.brand_ids.iter().map(|bid| model::CategoryBrand {
        brand_id: bid.clone(),
        category_id: r.category_id.clone(),
    });

    q.push_values(args, |mut b, cb| {
        b.push_bind(cb.brand_id).push_bind(cb.category_id);
    });

    let r = q.build().execute(e).await?;
    return Ok(r.rows_affected());
}

/// 分类品牌列表
pub async fn list_with_brands(
    conn: &sqlx::PgPool,
    r: &model::ListCategoryWithBrandsRequest,
) -> crate::Result<super::Paginate<model::CategoryWithBrands>> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, "name", parent, "path", "level", dateline, is_del, brand_ids, brand_names, brand_logos, brand_is_dels, brand_datelines, brand_names_str FROM v_category_with_brands WHERE 1=1"#,
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_category_with_brands WHERE 1=1");

    if let Some(name) = &r.name {
        let sql = " AND name ILIKE ";
        let arg = format!("%{}%", name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if let Some(parent) = &r.parent {
        let sql = " AND parent=";

        q.push(sql).push_bind(parent);
        qc.push(sql).push_bind(parent);
    }

    if let Some(level) = &r.level {
        let sql = " AND level=";

        q.push(sql).push_bind(level);
        qc.push(sql).push_bind(level);
    }

    if let Some(brand_name) = &r.brand_name {
        let sql = " AND brand_names_str ILIKE ";
        let arg = format!("%,%{}%,%", brand_name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    q.push(" ORDER BY id DESC");
    q.push(" LIMIT ")
        .push_bind(r.paginate.page_size())
        .push(" OFFSET ")
        .push_bind(r.paginate.offset());

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(super::Paginate::quick(&r.paginate, &count, data))
}

/// 品牌分类列表
pub async fn list_with_categoies(
    conn: &sqlx::PgPool,
    r: &model::ListBrandWithCategoriesRequest,
) -> crate::Result<super::Paginate<model::BrandWithCategoies>> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT brand_id, brand_name, brand_logo, brand_is_del, brand_dateline, ids, names, names_str, parents, levels, paths, datelines, is_dels FROM v_brand_with_categoies WHERE 1=1"#,
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_brand_with_categoies WHERE 1=1");

    if let Some(is_del) = &r.is_del {
        let sql = " AND brand_is_del = ";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if let Some(category_name) = &r.category_name {
        let sql = " AND names_str ILIKE ";
        let arg = format!("%,%{}%,%", category_name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(name) = &r.name {
        let sql = " AND brand_name ILIKE ";
        let arg = format!("%{}%", name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    q.push(" ORDER BY brand_id DESC");
    q.push(" LIMIT ")
        .push_bind(r.paginate.page_size())
        .push(" OFFSET ")
        .push_bind(r.paginate.offset());

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(super::Paginate::quick(&r.paginate, &count, data))
}

/// 查找带品牌信息的分类
pub async fn find_category<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::FindCategoryWithBrandsRequest,
) -> Result<Option<model::CategoryWithBrands>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, "name", parent, "path", "level", dateline, is_del, brand_ids, brand_names, brand_logos, brand_is_dels, brand_datelines, brand_names_str FROM v_category_with_brands WHERE 1=1"#,
    );
    match &r.by {
        model::FindCategoryBy::ID(id) => q.push(" AND id=").push_bind(id),
        model::FindCategoryBy::NameAndParent(nap) => q
            .push(" AND (name=")
            .push_bind(&nap.name)
            .push(" AND parent=")
            .push_bind(&nap.parent)
            .push(")"),
    };

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del = ").push_bind(is_del);
    }

    if let Some(level) = &r.level {
        q.push(" AND level = ").push_bind(level);
    }

    if let Some(brand_name) = &r.brand_name {
        q.push(" AND brand_names_str ILIKE ")
            .push_bind(format!("%,%{}%,%", brand_name));
    }

    q.build_query_as().fetch_optional(e).await
}

/// 查找带分类信息的品牌
pub async fn find_brand<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::FindBrandWithCategoriesRequest,
) -> Result<Option<model::BrandWithCategoies>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT brand_id, brand_name, brand_logo, brand_is_del, brand_dateline, ids, names, names_str, parents, levels, paths, datelines, is_dels FROM v_brand_with_categoies WHERE 1=1"#,
    );
    match &r.by {
        model::BrandFindBy::ID(id) => q.push(" AND brand_id=").push_bind(id),
        model::BrandFindBy::Name(name) => q.push(" AND name=").push_bind(name),
    };

    if let Some(is_del) = &r.is_del {
        q.push(" AND brand_is_del = ").push_bind(is_del);
    }

    if let Some(category_name) = &r.category_name {
        q.push(" AND names_str ILIKE ")
            .push_bind(format!("%,%{}%,%", category_name));
    }

    q.build_query_as().fetch_optional(e).await
}

/// 分类树，含品牌信息
pub async fn tree<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::CategoryTreeRequest,
) -> Result<Vec<model::Tree>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, "name", parent, "path", "level", dateline, is_del, brand_ids, brand_names, brand_logos, brand_is_dels, brand_datelines, brand_names_str, fullname FROM v_tree WHERE 1=1"#,
    );
    match &r.by {
        model::CategoryTreeBy::Parent(parent) => q.push(" AND parent=").push_bind(parent),
        model::CategoryTreeBy::Path(path) => {
            let arg = if path.is_empty() {
                format!("//%/")
            } else {
                format!("//%{}/%/", path)
            };
            q.push(" AND path LIKE ").push_bind(arg)
        }
    };

    if let Some(name) = &r.name {
        let sql = " AND name ILIKE ";
        let param = format!("%{}%", name);

        q.push(sql).push_bind(param);
    }

    if let Some(level) = &r.level {
        let sql = " AND level =";
        q.push(sql).push_bind(level);
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.push(" ORDER BY path");

    q.build_query_as().fetch_all(e).await
}

/// 清空品牌的分类
pub async fn clear<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::ClearCategoryBrandsRequest,
) -> Result<u64, sqlx::Error> {
    let r = sqlx::query("DELETE FROM category_brands WHERE category_id = $1")
        .bind(&r.category_id)
        .execute(e)
        .await?;
    return Ok(r.rows_affected());
}

#[cfg(test)]
mod test {
    use crate::{db, model};

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
        let cate_id = "ckkfod4drfam60t44b5g".to_string();
        let brand_ids = vec![
            "ckkfkpsdrfak8jh3sveg",
            "ckkfkpsdrfak8jh3svf0",
            "ckkfkpsdrfak8jh3svfg",
            // "ckkfkpsdrfak8jh3svg0",
            // "ckkfkpsdrfak8jh3svgg",
            "ckkfkpsdrfak8jh3svh0",
            "ckkfkpsdrfak8jh3svhg",
        ];
        let r = model::SetCategoryBrandRequest {
            category_id: cate_id,
            brand_ids: brand_ids.iter().map(|&bid| bid.to_string()).collect(),
        };
        let r = match super::set(&mut *tx, &r).await {
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

        let r = super::clear(
            &conn,
            &model::ClearCategoryBrandsRequest {
                category_id: category_id.to_string(),
            },
        )
        .await
        .unwrap();
        assert!(r > 0);
    }

    #[tokio::test]
    async fn test_db_list_with_brands() {
        let r = model::ListCategoryWithBrandsRequest {
            paginate: db::PaginateRequest {
                page: 0,
                page_size: 3,
            },
            name: Some("分类".to_string()),
            is_del: Some(false),
            parent: Some("".to_string()),
            level: Some(model::CategoryLevel::Level1),
            brand_name: Some("Rust".to_string()),
        };
        let conn = get_conn().await;
        let p = super::list_with_brands(&conn, &r).await.unwrap();
        println!("{}, {}", p.total_page, p.total);
        for cb in p.data.iter() {
            println!("{:?}", cb);
        }
    }

    #[tokio::test]
    async fn test_db_list_with_categoies() {
        let r = model::ListBrandWithCategoriesRequest {
            paginate: db::PaginateRequest {
                page: 0,
                page_size: 3,
            },
            name: Some("sql".to_string()),
            is_del: Some(false),
            category_name: Some("分类".to_string()),
        };
        let conn = get_conn().await;
        let p = super::list_with_categoies(&conn, &r).await.unwrap();

        println!("{}, {}", p.total, p.total_page);

        for bc in p.data.iter() {
            println!("{:?}", bc);
        }
    }

    #[tokio::test]
    async fn test_db_tree_with_brands() {
        let r = model::CategoryTreeRequest {
            // by: model::CategoryTreeBy::Parent("cji1llcdrfap1bhmp74g".to_string()),
            by: model::CategoryTreeBy::Path("cji1llcdrfap1bhmp74g".to_string()),
            is_del: Some(false),
            // level: Some(model::CategoryLevel::Level3),
            level: None,
            name: None,
        };
        let conn = get_conn().await;
        let trees = super::tree(&conn, &r).await.unwrap();

        for tree in trees.iter() {
            println!("{:?}", tree);
        }

        println!("{}", trees.len());
    }

    #[tokio::test]
    async fn test_db_find_category_with_brands() {
        let r = model::FindCategoryWithBrandsRequest {
            // by: model::FindCategoryBy::ID("cji1llcdrfap1bhmp76g".to_string()),
            by: model::FindCategoryBy::NameAndParent(model::CategoryNameAndParentRequest {
                name: "一级分类2".to_string(),
                parent: "".to_string(),
            }),
            is_del: Some(false),
            level: Some(model::CategoryLevel::Level1),
            brand_name: Some("Rust".to_string()),
        };
        let conn = get_conn().await;
        let cb = super::find_category(&conn, &r).await.unwrap();
        println!("{:?}", cb);
    }

    #[tokio::test]
    async fn test_db_find_brand_with_categoies() {
        let by = model::BrandFindBy::ID("ckkfkpsdrfak8jh3svh0".to_string());
        let r = model::FindBrandWithCategoriesRequest {
            by,
            is_del: Some(false),
            category_name: Some("分类".to_string()),
        };
        let conn = get_conn().await;
        let bc = super::find_brand(&conn, &r).await.unwrap();

        println!("{:?}", bc);
    }
}
