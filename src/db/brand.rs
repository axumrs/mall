use crate::{model, utils::id, Error, Result};

pub async fn exists(conn: &sqlx::PgPool, name: &str, id: Option<String>) -> Result<bool> {
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

pub async fn create(conn: &sqlx::PgPool, m: &model::Brand) -> Result<String> {
    let id = id::new();
    sqlx::query("INSERT INTO brands (id, name, logo, is_del,dateline) VALUES ($1, $2, $3, $4,$5)")
        .bind(&id)
        .bind(&m.name)
        .bind(&m.logo)
        .bind(&m.is_del)
        .bind(&m.dateline)
        .execute(conn)
        .await
        .map_err(Error::from)?;
    Ok(id)
}

pub async fn edit(conn: &sqlx::PgPool, m: &model::Brand) -> Result<u64> {
    let aff = sqlx::query("UPDATE brands SET name=$1, logo=$2 WHERE id=$3")
        .bind(&m.name)
        .bind(&m.logo)
        .bind(&m.id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn del_or_restore(conn: &sqlx::PgPool, id: String, is_del: bool) -> Result<u64> {
    super::del_or_restore(conn, "brands", id, is_del).await
}

pub async fn find(
    conn: &sqlx::PgPool,
    r: &model::BrandFindRequest,
) -> Result<Option<model::Brand>> {
    let mut q =
        sqlx::QueryBuilder::new("SELECT id, name, logo, is_del,dateline FROM brands WHERE 1=1");

    match r.by {
        model::BrandFindBy::ID(ref id) => q.push(" AND id=").push_bind(id),
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
    conn: &sqlx::PgPool,
    r: &model::BrandListRequest,
) -> Result<super::Paginate<model::Brand>> {
    let mut q =
        sqlx::QueryBuilder::new("SELECT id, name, logo, is_del,dateline FROM brands WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM brands WHERE 1=1");

    if let Some(name) = &r.name {
        let sql = " AND name ILIKE ";
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
        .push_bind(r.paginate.page_size())
        .push(" OFFSET ")
        .push_bind(r.paginate.offset());

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
