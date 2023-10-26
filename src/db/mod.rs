pub mod address;
pub mod banner;
pub mod brand;
pub mod cart;
pub mod category;
pub mod category_brand;
pub mod goods;
pub mod goods_attr;
pub mod paginate;
pub mod user;

pub use paginate::*;

pub async fn del_or_restore(
    conn: &sqlx::PgPool,
    table: &str,
    id: String,
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
