use crate::{model, utils::password, Error, Result};

pub async fn create(
    conn: &sqlx::MySqlPool,
    m: &model::User,
    sf: &mut snowflake::SnowflakeIdGenerator,
) -> Result<u64> {
    let id = sf.real_time_generate() as u64;
    sqlx::query("INSERT INTO users (id, email, password, nickname, status, dateline, is_del) VALUES(?,?,?,?,?,?,?)")
    .bind(&id)
    .bind(&m.email)
    .bind(&m.password)
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
