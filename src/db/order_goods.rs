use crate::{model, utils};

pub async fn create<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::OrderGoods,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();

    sqlx::query("INSERT INTO order_goods (id, order_id, goods_id, goods_sku, goods_snapshot, num, price) VALUES($1,$2,$3,$4,$5,$6,$7)")
    .bind(&id)
    .bind(&m.order_id)
    .bind(&m.goods_id)
    .bind(&m.goods_sku)
    .bind(&m.goods_snapshot)
    .bind(&m.num)
    .bind(&m.price)
    .execute(e).await?;

    Ok(id)
}
