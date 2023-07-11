pub mod paginate;
pub mod user;

pub async fn del_or_restore(
    conn: &sqlx::MySqlPool,
    table: &str,
    id: u64,
    is_del: bool,
) -> crate::Result<u64> {
    let mut q = sqlx::QueryBuilder::new("UPDATE ");
    q.push(table)
        .push(" SET is_del=")
        .push_bind(is_del)
        .push(" WHERE id=")
        .push_bind(id);

    let aff = q
        .build()
        .execute(conn)
        .await
        .map_err(crate::Error::from)?
        .rows_affected();
    Ok(aff)
}
