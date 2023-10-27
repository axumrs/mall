use crate::{model, utils, Error};

/// 创建支付
pub async fn create<'a>(
    e: impl sqlx::PgExecutor<'a>,
    p: &'a model::Pay,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();

    sqlx::query("INSERT INTO pays (id, order_id, status, tx_id, amount, dateline, timeout_until_dateline, done_dateline) VALUES($1, $2, $3, $4, $5, $6, $7, $8)")
    .bind(&id)
    .bind(&p.order_id)
    .bind(&p.status)
    .bind(&p.tx_id)
    .bind(&p.amount)
    .bind(&p.dateline)
    .bind(&p.timeout_until_dateline)
    .bind(&p.done_dateline)
    .execute(e).await?;

    Ok(id)
}

/// 查找支付
pub async fn find<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &'a model::FindPayRequest,
) -> Result<Option<model::Pay>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, order_id, status, tx_id, amount, dateline, timeout_until_dateline, done_dateline FROM pays WHERE 1=1");
    q.push(" AND id=").push_bind(&r.id);

    // if let Some(user_id) = &r.user_id {
    //     // TODO: 关联订单表查询
    // }

    q.build_query_as().fetch_optional(e).await
}

/// 订单支付列表
pub async fn list4order<'a>(
    conn: &'a sqlx::PgPool,
    r: &'a model::ListPayForOrderRequest,
) -> crate::Result<model::ListPayForOrderResponse> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, order_id, status, tx_id, amount, dateline, timeout_until_dateline, done_dateline FROM pays WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM pays WHERE 1=1");

    q.push(" AND order_id=").push_bind(&r.order_id);
    qc.push(" AND order_id=").push_bind(&r.order_id);

    // if let Some(user_id) = &r.user_id {
    //     // TODO: 关联订单表查询
    // }

    if let Some(status) = &r.status {
        q.push(" AND status=").push_bind(status);
    }

    q.push(" ORDER BY id DESC");

    if let Some(top) = r.top {
        q.push(" LIMIT ").push_bind(top);
    }

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let ls: Vec<model::Pay> = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    let resp = if r.pined_done {
        let done_pay = ls
            .iter()
            .filter(|p| match p.status {
                model::PayStatus::Done => true,
                _ => false,
            })
            .cloned()
            .next();
        if done_pay.is_some() {
            let done_pay = done_pay.unwrap();
            let mut tmp = Vec::with_capacity(ls.len());
            let done_pay_id = done_pay.id.clone();

            tmp.push(done_pay);

            ls.iter()
                .filter(|p| &p.id != &done_pay_id)
                .cloned()
                .for_each(|p| tmp.push(p));

            model::ListPayForOrderResponse {
                total: count.0,
                pays: tmp,
            }
        } else {
            model::ListPayForOrderResponse {
                total: count.0,
                pays: ls,
            }
        }
    } else {
        model::ListPayForOrderResponse {
            total: count.0,
            pays: ls,
        }
    };
    Ok(resp)
}

/// 完成支付
pub async fn done<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &'a model::PayDoneRequest,
) -> Result<Option<model::Pay>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("UPDATE pays SET tx_id=");
    q.push_bind(&r.tx_id)
        .push(", status=")
        .push_bind(model::PayStatus::Done)
        .push(", done_dateline=")
        .push_bind(chrono::Local::now())
        .push(" WHERE id=")
        .push_bind(&r.id);

    q.push(" AND status=").push_bind(model::PayStatus::Paying);

    // if let Some(user_id) = &r.user_id {
    //     // TODO: 关联订单表查询
    // }

    q.push(" RETURNING *");
    q.build_query_as().fetch_optional(e).await
}

/// 订单是否正在支付
pub async fn order_is_paying<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &'a model::OrderIsPayingRequest,
) -> Result<bool, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM pays WHERE 1=1");
    q.push(" AND order_id=").push_bind(&r.order_id);

    // if let Some(user_id) = &r.user_id {
    //     // TODO: 关联订单表查询
    // }

    q.push(" AND status=").push_bind(model::PayStatus::Paying);
    // q.push(" AND timeout_until_dateline <=")
    //     .push_bind(chrono::Local::now());

    let count: (i64,) = q.build_query_as().fetch_one(e).await?;

    Ok(count.0 > 0)
}

#[cfg(test)]
mod test {

    use crate::model::{self, U32};

    // const TEST_USER_ID: &str = "cj7kfuel6bcqn8bicrng";
    const TEST_ORDER_ID: &str = "ckt123cdrfal3mj04d30";
    const TEST_PAY_ID: &str = "cktii4sdrfan75sn19ig";

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_db_create_pay() {
        let dateline = chrono::Local::now();

        let p = model::Pay {
            order_id: TEST_ORDER_ID.into(),
            status: model::PayStatus::Paying,
            amount: U32::from_yuan(10),
            dateline,
            timeout_until_dateline: dateline + chrono::Duration::minutes(15), // 15分钟
            ..Default::default()
        };
        let conn = get_conn().await;

        let id = super::create(&conn, &p).await.unwrap();

        assert_eq!(id.is_empty(), false);
        println!("id: {}", id);
    }

    #[tokio::test]
    async fn test_db_find_pay() {
        let r = model::FindPayRequest {
            id: TEST_PAY_ID.into(),
            user_id: None,
        };
        let conn = get_conn().await;

        let pay = super::find(&conn, &r).await.unwrap();

        println!("{:?}", pay);
    }

    #[tokio::test]
    async fn test_db_list_pay_for_order() {
        let r = model::ListPayForOrderRequest {
            user_id: None,
            order_id: TEST_ORDER_ID.into(),
            status: None,
            top: Some(10),
            pined_done: true,
        };
        let conn = get_conn().await;

        let ls = super::list4order(&conn, &r).await.unwrap();

        println!("{}, {},  {:?}", ls.total, ls.pays.len(), ls.pays);
    }
    #[tokio::test]
    async fn test_db_order_ispaying() {
        let r = model::OrderIsPayingRequest {
            user_id: None,
            order_id: TEST_ORDER_ID.into(),
        };
        let conn = get_conn().await;

        let is_paying = super::order_is_paying(&conn, &r).await.unwrap();

        println!("{}", is_paying);
    }
    #[tokio::test]
    async fn test_db_order_pay_done() {
        let r = model::PayDoneRequest {
            id: TEST_PAY_ID.into(),
            user_id: None,
            tx_id: "FOOBAR".into(),
        };
        let conn = get_conn().await;

        let pay = super::done(&conn, &r).await.unwrap();

        println!("{:?}", pay);
    }
}
