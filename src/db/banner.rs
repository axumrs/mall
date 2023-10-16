use crate::{model, utils, Error, Result};

use super::Paginate;

/// 创建轮播图
pub async fn create(conn: &sqlx::PgPool, b: &model::Banner) -> Result<String> {
    let id = utils::id::new();

    sqlx::query(
        "INSERT INTO banners (id, img, url, sort, title, is_del) VALUES($1, $2, $3, $4, $5, $6)",
    )
    .bind(&id)
    .bind(&b.img)
    .bind(&b.url)
    .bind(&b.sort)
    .bind(&b.title)
    .bind(&b.is_del)
    .execute(conn)
    .await
    .map_err(Error::from)?;

    Ok(id)
}

/// 修改轮播图
pub async fn edit(conn: &sqlx::PgPool, b: &model::Banner) -> Result<u64> {
    let aff = sqlx::query("UPDATE banners SET img=$1, url=$2, sort=$3, title=$4 WHERE id=$5")
        .bind(&b.img)
        .bind(&b.url)
        .bind(&b.sort)
        .bind(&b.title)
        .bind(&b.id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff)
}

// 删除或还原轮播图
pub async fn del_or_restore(conn: &sqlx::PgPool, id: String, is_del: bool) -> Result<u64> {
    super::del_or_restore(conn, "banners", id, is_del).await
}

/// 查找轮播图
pub async fn find(
    conn: &sqlx::PgPool,
    r: &model::FindBannerRequest,
) -> Result<Option<model::Banner>> {
    let mut q =
        sqlx::QueryBuilder::new("SELECT id, img, url, sort, title, is_del FROM banners WHERE 1=1");

    q.push(" AND id=").push_bind(&r.id);

    if let Some(title) = &r.title {
        q.push(" AND title ILIKE ")
            .push_bind(format!("%{}%", title));
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    let b = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(b)
}

/// 轮播图列表
pub async fn list(
    conn: &sqlx::PgPool,
    r: &model::ListBannerRequest,
) -> Result<Paginate<model::Banner>> {
    let mut q =
        sqlx::QueryBuilder::new("SELECT id, img, url, sort, title, is_del FROM banners WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM banners WHERE 1=1");

    if let Some(title) = &r.title {
        let sql = " AND title ILIKE ";
        let param = format!("%{}%", title);

        q.push(sql).push_bind(param.clone());
        qc.push(sql).push_bind(param);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if r.order_by_sort {
        q.push(" ORDER BY sort DESC");
    } else {
        q.push(" ORDER BY id DESC");
    }

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

    Ok(Paginate::quick(&r.paginate, &count, data))
}

/// 前N个轮播图
pub async fn top(conn: &sqlx::PgPool, r: &model::TopNBannerRequest) -> Result<Vec<model::Banner>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id, img, url, sort, title, is_del FROM banners WHERE is_del = false",
    );
    if let Some(title) = &r.title {
        let sql = " AND title ILIKE ";
        let param = format!("%{}%", title);

        q.push(sql).push_bind(param);
    }

    if r.order_by_id {
        q.push(" ORDER BY id DESC");
    } else {
        q.push(" ORDER BY sort DESC");
    }

    q.push(" LIMIT ").push_bind(r.num);

    let ls = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(ls)
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
    async fn test_db_create_banner() {
        let conn = get_conn().await;
        let b = model::Banner {
            img: "https://file.axum.rs/asset/logo.png".to_string(),
            url: "https://axum.rs".to_string(),
            title: "AXUM中文网".to_string(),
            ..Default::default()
        };
        let id = super::create(&conn, &b).await.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn test_db_batch_create_banner() {
        let conn = get_conn().await;
        const NUM: usize = 5;

        let mut ids = vec![];
        for i in 0..NUM {
            let b = model::Banner {
                img: "https://file.axum.rs/asset/logo.png".to_string(),
                url: "https://axum.rs".to_string(),
                title: format!("AXUM中文网-{}", i),
                sort: i as i32,
                ..Default::default()
            };
            let id = super::create(&conn, &b).await.unwrap();
            ids.push(id);
        }

        assert!(ids.len() == NUM);
        assert!(ids.iter().all(|id| !id.is_empty()));
    }

    #[tokio::test]
    async fn test_db_edit_banner() {
        let conn = get_conn().await;

        let b = model::Banner {
            id: "ckm9vdsdrfaqj2sd4qog".to_string(),
            img: "https://file.axum.rs/asset/logo.png".to_string(),
            url: "https://axum.rs".to_string(),
            title: "AXUM中文网-EDITED".to_string(),
            sort: 0,
            ..Default::default()
        };

        let aff = super::edit(&conn, &b).await.unwrap();

        assert!(aff == 1);
    }

    #[tokio::test]
    async fn test_db_del_or_restore_banner() {
        let conn = get_conn().await;
        let aff = super::del_or_restore(&conn, "ckm9vdsdrfaqj2sd4qog".to_string(), false)
            .await
            .unwrap();
        assert!(aff == 1);
    }

    #[tokio::test]
    async fn test_db_find_banner() {
        let conn = get_conn().await;
        let r = model::FindBannerRequest {
            id: "ckm9vdsdrfaqj2sd4qog".to_string(),
            title: Some("AXUM".to_string()),
            is_del: Some(false),
        };
        let b = super::find(&conn, &r).await.unwrap();
        assert!(b.is_some());
        assert!(b.unwrap().id == r.id);
    }

    #[tokio::test]
    async fn test_db_list_banner() {
        let conn = get_conn().await;
        let r = model::ListBannerRequest {
            title: Some("AXUM".to_string()),
            is_del: Some(false),
            order_by_sort: false,
            paginate: db::PaginateRequest {
                page: 1,
                page_size: 3,
            },
        };
        let p = super::list(&conn, &r).await.unwrap();

        for b in p.data.iter() {
            println!("{:?}", b);
        }

        println!("{}, {}", p.total, p.total_page);
    }

    #[tokio::test]
    async fn test_db_top_n_banner() {
        let conn = get_conn().await;
        let r = model::TopNBannerRequest {
            title: Some("AXUM".to_string()),
            order_by_id: false,
            num: 3,
        };
        let ls = super::top(&conn, &r).await.unwrap();

        for b in ls.iter() {
            println!("{:?}", b);
        }
    }
}
