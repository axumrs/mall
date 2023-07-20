use crate::{model, utils::password, Error, Result};

pub async fn create(
    conn: &sqlx::MySqlPool,
    m: &model::User,
    sf: &mut snowflake::SnowflakeIdGenerator,
) -> Result<u64> {
    let id = sf.real_time_generate() as u64;
    let pwd = password::hash(&m.password)?;
    sqlx::query("INSERT INTO users (id, email, password, nickname, status, dateline, is_del) VALUES(?,?,?,?,?,?,?)")
    .bind(&id)
    .bind(&m.email)
    .bind(&pwd)
    .bind(&m.nickname)
    .bind(&m.status)
    .bind(&m.dateline)
    .bind(&m.is_del)
        .execute(conn)
        .await
        .map_err(Error::from)?;
    Ok(id)
}

pub async fn exists(
    conn: &sqlx::MySqlPool,
    email: String,
    nickname: Option<String>,
    id: Option<u64>,
) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM users WHERE 1=1");

    if let Some(nickname) = nickname {
        q.push(" AND (email=")
            .push_bind(&email)
            .push(" OR nickname=")
            .push_bind(nickname)
            .push(")");
    } else {
        q.push(" AND email=").push_bind(&email);
    }

    if let Some(id) = id {
        q.push(" AND id<>").push_bind(id);
    }

    let count: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    Ok(count.0 > 0)
}

pub async fn edit(conn: &sqlx::MySqlPool, m: &model::User) -> Result<u64> {
    let aff_row = sqlx::query("UPDATE users SET nickname=? WHERE id=?")
        .bind(&m.nickname)
        .bind(&m.id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff_row)
}

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u64, is_del: bool) -> Result<u64> {
    super::del_or_restore(conn, "users", id, is_del).await
}

pub async fn change_status(
    conn: &sqlx::MySqlPool,
    id: u64,
    status: model::UserStatus,
) -> Result<u64> {
    let aff_row = sqlx::query("UPDATE users SET status=? WHERE id=?")
        .bind(&status)
        .bind(&id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff_row)
}

pub async fn change_password(
    conn: &sqlx::MySqlPool,
    id: u64,
    new_password: String,
    password: Option<String>,
) -> Result<u64> {
    let u = find(
        conn,
        &model::UserFindRequest {
            by: model::UserFindBy::ID(id),
            is_del: None,
            status: None,
        },
    )
    .await?;
    if u.is_none() {
        return Err(Error::not_found());
    }

    let u = u.unwrap();

    if let Some(pwd) = password {
        if !password::verify(&pwd, &u.password)? {
            return Err(Error::incorrect_auth_with("密码错误"));
        }
    }

    let pwd = password::hash(&new_password)?;
    let aff = sqlx::query("UPDATE users SET password=? WHERE id=?")
        .bind(&pwd)
        .bind(&id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn find(
    conn: &sqlx::MySqlPool,
    r: &model::UserFindRequest,
) -> Result<Option<model::User>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id, email, password, nickname, status, dateline, is_del FROM users WHERE 1=1",
    );
    match r.by {
        model::UserFindBy::Email(ref email) => q.push(" AND email=").push_bind(email),
        model::UserFindBy::ID(ref id) => q.push(" AND id=").push_bind(id),
    };

    if let Some(status) = &r.status {
        q.push(" AND status=").push_bind(status);
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    r: &model::UserListRequest,
) -> Result<super::Paginate<model::User>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id, email, password, nickname, status, dateline, is_del FROM users WHERE 1=1",
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM users WHERE 1=1");

    if let Some(email) = &r.email {
        let sql = " AND email LIKE ";
        let param = format!("%{}%", email);
        q.push(sql).push_bind(param.clone());
        qc.push(sql).push_bind(param);
    }

    if let Some(nickname) = &r.nickname {
        let sql = " AND nickname LIKE ";
        let param = format!("%{}%", nickname);
        q.push(sql).push_bind(param.clone());
        qc.push(sql).push_bind(param);
    }

    if let Some(status) = &r.status {
        let sql = " AND status = ";
        q.push(sql).push_bind(status);
        qc.push(sql).push_bind(status);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del = ";
        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if let Some(date_range) = &r.date_range {
        let sql = " AND dateline BETWEEN ";
        let sql_end = " AND ";
        q.push(sql)
            .push_bind(&date_range.start)
            .push(sql_end)
            .push_bind(&date_range.end);
        qc.push(sql)
            .push_bind(&date_range.start)
            .push(sql_end)
            .push_bind(&date_range.end);
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

    let list: Vec<model::User> = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(super::Paginate::quick(&r.paginate, &count, list))
}
