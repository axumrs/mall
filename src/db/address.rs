use crate::{model, utils, Error};

use super::Paginate;

/// 是否有默认地址
pub async fn has_default<'a>(
    e: impl sqlx::PgExecutor<'a>,
    user_id: &'a str,
) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        "SELECT count(*) FROM address WHERE user_id =$1 AND is_del=false AND is_default=true",
    )
    .bind(user_id)
    .fetch_one(e)
    .await?;
    Ok(count.0 > 0)
}

/// 添加地址
pub async fn create<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::Address,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();

    sqlx::query("INSERT INTO address (id, user_id, is_default, consignee, phone, address, province, city, county, post_code, is_del, dateline) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)")
    .bind(&id)
    .bind(&m.user_id)
    .bind(&m.is_default)
    .bind(&m.detail.consignee)
    .bind(&m.detail.phone)
    .bind(&m.detail.address)
    .bind(&m.detail.province)
    .bind(&m.detail.city)
    .bind(&m.detail.county)
    .bind(&m.detail.post_code)
    .bind(&m.is_del)
    .bind(&m.dateline)
    .execute(e).await?;

    Ok(id)
}

/// 修改地址
pub async fn edit<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::Address,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE address SET is_default=$1, consignee=$2, phone=$3, address=$4, province=$5, city=$6, county=$7, post_code=$8 WHERE id=$9 AND user_id=$10")
    .bind(&m.is_default)
    .bind(&m.detail.consignee)
    .bind(&m.detail.phone)
    .bind(&m.detail.address)
    .bind(&m.detail.province)
    .bind(&m.detail.city)
    .bind(&m.detail.county)
    .bind(&m.detail.post_code)
    .bind(&m.id)
    .bind(&m.user_id)
    .execute(e).await?.rows_affected();

    Ok(aff)
}

/// 删除或还原
pub async fn del_or_restore<'a>(
    e: impl sqlx::PgExecutor<'a>,
    id: &str,
    is_del: bool,
    user_id: Option<&'a str>,
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("UPDATE address SET is_del =");
    q.push_bind(is_del).push(" WHERE id=").push_bind(id);

    if let Some(user_id) = user_id {
        q.push(" AND user_id = ").push_bind(user_id);
    }

    let aff = q.build().execute(e).await?.rows_affected();

    Ok(aff)
}

/// 查找
pub async fn find<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::FindAddressRequest,
) -> Result<Option<model::Address>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, is_default, consignee, phone, address, province, city, county, post_code, is_del, dateline FROM address WHERE 1=1");

    match &r.by {
        model::FindAddressBy::ID(id) => q.push(" AND id=").push_bind(id),
        model::FindAddressBy::IsDefault => q.push(" AND is_default=true"),
    };

    if let Some(user_id) = &r.user_id {
        q.push(" AND user_id=").push_bind(user_id);
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.push(" LIMIT 1");

    q.build_query_as().fetch_optional(e).await
}

/// 列表
pub async fn list(
    conn: &sqlx::PgPool,
    r: &model::ListAddressRequest,
) -> crate::Result<Paginate<model::Address>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, is_default, consignee, phone, address, province, city, county, post_code, is_del, dateline FROM address WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM address WHERE 1=1");

    if let Some(user_id) = &r.user_id {
        let sql = " AND user_id=";

        q.push(sql).push_bind(user_id);
        qc.push(sql).push_bind(user_id);
    }

    if let Some(consignee) = &r.consignee {
        let sql = " AND consignee ILIKE ";
        let arg = format!("%{}%", consignee);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(phone) = &r.phone {
        let sql = " AND phone ILIKE ";
        let arg = format!("%{}%", phone);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(adr) = &r.address {
        let sql = " AND address ILIKE ";
        let arg = format!("%{}%", adr);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del = ";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
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

/// 清除默认
pub async fn clear_default<'a>(
    e: impl sqlx::PgExecutor<'a>,
    user_id: &'a str,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE address SET is_default = false WHERE user_id =$1")
        .bind(user_id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 设置默认
///
/// 通常需要先调用 `clear_default()`
pub async fn set_default<'a>(
    e: impl sqlx::PgExecutor<'a>,
    id: &'a str,
    user_id: &'a str,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query(
        "UPDATE address SET is_default = true WHERE id=$1 AND user_id=$2 AND is_del=false",
    )
    .bind(id)
    .bind(user_id)
    .execute(e)
    .await?
    .rows_affected();

    Ok(aff)
}

/// 获取默认地址
pub async fn get_default<'a>(
    e: impl sqlx::PgExecutor<'a>,
    user_id: &'a str,
) -> Result<Option<model::Address>, sqlx::Error> {
    sqlx::query_as("SELECT id, user_id, is_default, consignee, phone, address, province, city, county, post_code, is_del, dateline FROM address WHERE user_id =$1 AND is_del=false ORDER BY is_default DESC,id ASC LIMIT 1").bind(user_id).fetch_optional(e).await
}

#[cfg(test)]
mod test {
    use crate::model;

    const TEST_USER_ID: &str = "cj7kfuel6bcqn8bicrng";

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_db_create_address() {
        let mut m = model::Address {
            user_id: TEST_USER_ID.into(),
            dateline: chrono::Local::now(),
            detail: model::AddressDetail {
                consignee: format!("张三"),
                phone: format!("13800138000"),
                address: format!("广园南路1024-512号"),
                province: format!("广东"),
                city: format!("广州"),
                county: format!("天河区"),
                post_code: format!("512345"),
            },
            ..Default::default()
        };
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        let has_default = match super::has_default(&mut *tx, &m.user_id).await {
            Ok(hd) => hd,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", e);
            }
        };

        m.is_default = !has_default;

        let id = match super::create(&mut *tx, &m).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", e);
            }
        };

        tx.commit().await.unwrap();

        assert!(!id.is_empty());
        println!("{}", id);
    }

    #[tokio::test]
    async fn test_db_edit_address() {
        let m = model::Address {
            id: "ckss90sdrfar2m1feiq0".into(),
            user_id: TEST_USER_ID.into(),
            is_default: true,
            detail: model::AddressDetail {
                consignee: format!("张三1"),
                phone: format!("13800138001"),
                address: format!("广园南路1024-512号1"),
                province: format!("广东1"),
                city: format!("广州1"),
                county: format!("天河区1"),
                post_code: format!("512346"),
            },
            ..Default::default()
        };
        let conn = get_conn().await;

        let aff = super::edit(&conn, &m).await.unwrap();

        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_db_del_or_restore_address() {
        let conn = get_conn().await;
        let aff = super::del_or_restore(&conn, "ckss90sdrfar2m1feiq0", false, Some(TEST_USER_ID))
            .await
            .unwrap();
        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_db_find_address() {
        let r = model::FindAddressRequest {
            // by: model::FindAddressBy::ID("ckss90sdrfar2m1feiq0".into()),
            by: model::FindAddressBy::IsDefault,
            user_id: Some(TEST_USER_ID.into()),
            is_del: Some(false),
        };

        let conn = get_conn().await;
        let addr = super::find(&conn, &r).await.unwrap();

        println!("{:?}", addr);
    }

    #[tokio::test]
    async fn test_db_list_address() {
        let r = model::ListAddressRequest {
            paginate: crate::db::PaginateRequest {
                page: 0,
                page_size: 3,
            },
            user_id: Some(TEST_USER_ID.into()),
            consignee: Some("张三".into()),
            phone: Some("138".into()),
            address: Some("广园".into()),
            is_del: Some(false),
        };

        let conn = get_conn().await;
        let p = super::list(&conn, &r).await.unwrap();

        println!("{}, {}", p.total, p.total_page);

        for addr in p.data.iter() {
            println!("{:?}", addr);
        }
    }

    #[tokio::test]
    async fn test_db_set_default_address() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        if let Err(e) = super::clear_default(&mut *tx, TEST_USER_ID).await {
            tx.rollback().await.unwrap();
            panic!("{:?}", e);
        }

        let aff = match super::set_default(&mut *tx, "ckss9csdrfarvji2j8fg", TEST_USER_ID).await {
            Ok(aff) => aff,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", e);
            }
        };

        tx.commit().await.unwrap();

        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_db_get_default_address() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        // if let Err(e) = super::clear_default(&mut *tx, TEST_USER_ID).await {
        //     tx.rollback().await.unwrap();
        //     panic!("{:?}", e);
        // }

        let addr = match super::get_default(&mut *tx, TEST_USER_ID).await {
            Ok(addr) => addr,
            Err(e) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", e);
            }
        };

        tx.commit().await.unwrap();

        println!("{:?}", addr);
    }
}
