use crate::{model, utils};

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
    user_id: &'a str,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE carts SET num=$1 WHERE id=$2 AND user_id=$3")
        .bind(&item.num)
        .bind(&item.id)
        .bind(user_id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

#[cfg(test)]
mod test {
    use crate::model;

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_db_add_item_cart() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        let c = model::Cart {
            user_id: format!("{}", "cj7kfuel6bcqn8bicrng"),
            goods_id: format!("{}", "ckmivtsdrfagh6slh3tg"),
            // goods_sku: format!("{}", "米色-M-V领"),
            goods_sku: format!("{}", "米色-S-V领"),
            num: model::U32::new(1),
            // amount: model::U32::from_yuan(100),
            dateline: chrono::Local::now(),
            ..Default::default()
        };

        let id = match super::add_item(&mut *tx, &c).await {
            Ok(id) => id,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        let ls = match super::get(&mut *tx, &c.user_id).await {
            Ok(ls) => ls,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        tx.commit().await.unwrap();

        println!("id: {}, ls: {:?}", id, ls);
    }

    #[tokio::test]
    async fn test_db_get_items_cart() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();
        let ls = match super::get(&mut *tx, "cj7kfuel6bcqn8bicrng").await {
            Ok(ls) => ls,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };
        tx.commit().await.unwrap();
        println!("{:?}", ls)
    }

    #[tokio::test]
    async fn test_db_update_num_cart() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        let items = vec![model::CartItemNum {
            id: "cksauc4drfao8bsjqhsg".to_string(),
            num: model::U32::new(10),
        }];

        let mut affs = Vec::with_capacity(items.len());
        for i in items.iter() {
            let aff = match super::update_num(&mut *tx, i, "cj7kfuel6bcqn8bicrng").await {
                Ok(aff) => aff,
                Err(err) => {
                    tx.rollback().await.unwrap();
                    panic!("{:?}", err);
                }
            };
            affs.push(aff);
        }

        let ls = match super::get(&mut *tx, "cj7kfuel6bcqn8bicrng").await {
            Ok(ls) => ls,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        tx.commit().await.unwrap();

        println!("affs: {:?}, items: {:?}", affs, ls);
    }

    #[tokio::test]
    async fn test_db_remove_item_cart() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        let r = model::RemoveItemFromCartRequest {
            ids: vec!["cksauc4drfao8bsjqhsg".to_string()],
            user_id: "cj7kfuel6bcqn8bicrng".to_string(),
        };

        let removted = match super::remove_item(&mut *tx, &r).await {
            Ok(rm) => rm,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        let ls = match super::get(&mut *tx, &r.user_id).await {
            Ok(ls) => ls,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        tx.commit().await.unwrap();
        println!("remoted: {:?}, ls:{:?}", removted, ls)
    }

    #[tokio::test]
    async fn test_db_clear_item_cart() {
        let conn = get_conn().await;
        let mut tx = conn.begin().await.unwrap();

        let aff = match super::clear(&mut *tx, "cj7kfuel6bcqn8bicrng").await {
            Ok(a) => a,
            Err(err) => {
                tx.rollback().await.unwrap();
                panic!("{:?}", err);
            }
        };

        tx.commit().await.unwrap();

        println!("aff: {}", aff);
    }
}
