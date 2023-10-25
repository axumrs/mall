use crate::{model, utils};
use futures::StreamExt;

/// 添加到购物车
pub async fn add_item<'a>(
    e: impl sqlx::PgExecutor<'a>,
    item: &'a model::Cart,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();

    let hash = item.hash(&id);

    let sql = "INSERT INTO carts (id, user_id, goods_id, goods_sku, num, amount, hash, dateline) VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (hash) DO UPDATE SET num=EXCLUDED.num+1";
    sqlx::query(sql)
        .bind(&id)
        .bind(&item.user_id)
        .bind(&item.goods_id)
        .bind(&item.goods_sku)
        .bind(&item.num)
        .bind(&item.amount)
        .bind(&hash)
        .bind(&item.dateline)
        .execute(e)
        .await?;

    Ok(id)
}

/// 获取购物车
pub async fn get<'a>(
    e: impl sqlx::PgExecutor<'a>,
    user_id: &'a str,
) -> Result<Vec<model::Cart>, sqlx::Error> {
    let sql = "SELECT id, user_id, goods_id, goods_sku, num, amount, hash, dateline FROM carts WHERE user_id = $1";
    sqlx::query_as(sql).bind(user_id).fetch_all(e).await
}

/// 清空购物车
pub async fn clear<'a>(e: impl sqlx::PgExecutor<'a>, user_id: &'a str) -> Result<u64, sqlx::Error> {
    let sql = "DELETE FROM carts WHERE user_id = $1";
    let aff = sqlx::query(sql)
        .bind(user_id)
        .execute(e)
        .await?
        .rows_affected();

    Ok(aff)
}

/// 从购物车删除
pub async fn remove_item<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &model::RemoveItemFromCartRequest,
) -> Result<Vec<model::Cart>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("DELETE FROM carts WHERE user_id =");
    q.push_bind(&r.user_id);
    q.push(" AND id IN ");

    q.push_tuples(r.ids.iter(), |mut b, id| {
        b.push_bind(id);
    });

    q.push(" RETURNING *");

    q.build_query_as().fetch_all(e).await
}

/// 更新购物车数量
pub async fn update_num<'a>(
    e: impl sqlx::PgExecutor<'a>,
    item: &'a model::CartItemNum,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE carts SET num=$1 WHERE id=$2")
        .bind(&item.num)
        .bind(&item.id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}
