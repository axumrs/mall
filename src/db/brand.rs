use crate::{model, Error, Result};

pub async fn exists(conn: &sqlx::MySqlPool, name: &str, id: Option<u64>) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM brands WHERE name=");
    q.push_bind(name);

    if let Some(id) = id {
        q.push(" AND id <>").push_bind(id);
    }

    let count: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    Ok(count.0 > 0)
}

pub async fn create(
    conn: &sqlx::MySqlPool,
    m: &model::Brand,
    sf: &mut snowflake::SnowflakeIdGenerator,
) -> Result<u64> {
    let id = sf.real_time_generate() as u64;
    sqlx::query("INSERT INTO brands (id, name, logo, is_del) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&m.name)
        .bind(&m.logo)
        .bind(&m.is_del)
        .execute(conn)
        .await
        .map_err(Error::from)?;
    Ok(id)
}

pub async fn edit(conn: &sqlx::MySqlPool, m: &model::Brand) -> Result<u64> {
    let aff = sqlx::query("UPDATE brands SET name=?, logo=? WHERE id=?")
        .bind(&m.name)
        .bind(&m.logo)
        .bind(&m.id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u64, is_del: bool) -> Result<u64> {
    super::del_or_restore(conn, "brands", id, is_del).await
}

pub async fn find(
    conn: &sqlx::MySqlPool,
    r: &model::BrandFindRequest,
) -> Result<Option<model::Brand>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, name, logo, is_del FROM brands WHERE 1=1");

    match r.by {
        model::BrandFindBy::ID(id) => q.push(" AND id=").push_bind(id),
        model::BrandFindBy::Name(ref name) => q.push(" AND name=").push_bind(name),
    };

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

pub async fn list(
    conn: &sqlx::MySqlPool,
    r: &model::BrandListRequest,
) -> Result<super::Paginate<model::Brand>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, name, logo, is_del FROM brands WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM brands WHERE 1=1");

    if let Some(name) = &r.name {
        let sql = " AND name LIKE ";
        let param = format!("%{}%", name);

        q.push(sql).push_bind(param.clone());
        qc.push(sql).push_bind(param);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" LIMIT ")
        .push_bind(r.paginate.page_size)
        .push(" OFFSET ")
        .push_bind(r.paginate.page * r.paginate.page_size);

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    let ls = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(super::Paginate::quick(&r.paginate, &count, ls))
}
