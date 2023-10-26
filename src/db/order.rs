use crate::{model, utils, Error};

use super::Paginate;

/// 创建订单
pub async fn create<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::Order,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();
    sqlx::query("INSERT INTO orders (id, user_id, cart_id, sn, status, amount, total_num, freight, address, delivery_id, dateline, cancel_until_dateline, confirm_until_dateline, is_del) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)")
    .bind(&id)
    .bind(&m.user_id)
    .bind(&m.cart_id)
    .bind(&m.sn)
    .bind(&m.status)
    .bind(&m.amount)
    .bind(&m.total_num)
    .bind(&m.freight)
    .bind(&m.address)
    .bind(&m.delivery_id)
    .bind(&m.dateline)
    .bind(&m.cancel_until_dateline)
    .bind(&m.confirm_until_dateline)
    .bind(&m.is_del)
    .execute(e).await?;

    Ok(id)
}

/// 修改价格
pub async fn edit_amount<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::EditOrderAmountRequest,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE orders SET amount = $1 WHERE id=$2")
        .bind(&m.amount)
        .bind(&m.id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 修改收货地址
pub async fn edit_address<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::EditOrderAddressRequest,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query("UPDATE orders SET address = $1 WHERE id=$2")
        .bind(&m.address)
        .bind(&m.id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}
/// 修改状态
pub async fn edit_status<'a>(
    e: impl sqlx::PgExecutor<'a>,
    m: &'a model::EditOrderStatusRequest,
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("UPDATE orders SET status = ");
    q.push_bind(&m.status);
    q.push(" WHERE id=").push_bind(&m.id);

    if let Some(pre_status) = &m.pre_status {
        q.push(" AND status = ").push_bind(pre_status);
    }

    let aff = q.build().execute(e).await?.rows_affected();

    Ok(aff)
}
/// 删除或还原
pub async fn del_or_restore<'a>(
    e: impl sqlx::PgExecutor<'a>,
    id: &'a str,
    is_del: bool,
    user_id: Option<String>,
) -> Result<u64, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("UPDATE orders SET is_del=");
    q.push_bind(is_del).push(" WHERE id=").push_bind(id);

    if let Some(user_id) = user_id {
        q.push(" AND user_id = ").push_bind(user_id);
    }

    let aff = q.build().execute(e).await?.rows_affected();

    Ok(aff)
}

/// 查找
pub async fn find<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &'a model::FindOrderRequest,
) -> Result<Option<model::Order>, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, cart_id, sn, status, amount, total_num, freight, address, delivery_id, dateline, cancel_until_dateline, confirm_until_dateline, is_del FROM orders WHERE 1=1");

    match &r.by {
        model::FindOrderBy::ID(id) => q.push(" AND id=").push_bind(id),
        model::FindOrderBy::SN(sn) => q.push(" AND sn=").push_bind(sn),
    };

    if let Some(user_id) = &r.user_id {
        q.push(" AND user_id=").push_bind(user_id);
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.build_query_as().fetch_optional(e).await
}

/// 列表
pub async fn list<'a>(
    conn: &'a sqlx::PgPool,
    r: &'a model::ListOrderRequest,
) -> crate::Result<Paginate<model::Order>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, cart_id, sn, status, amount, total_num, freight, address, delivery_id, dateline, cancel_until_dateline, confirm_until_dateline, is_del FROM orders WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM orders WHERE 1=1");

    if let Some(user_id) = &r.user_id {
        let sql = " AND user_id=";

        q.push(sql).push_bind(user_id);
        qc.push(sql).push_bind(user_id);
    }

    if let Some(is_del) = &r.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if let Some(sn) = &r.sn {
        let sql = " AND sn ILIKE ";
        let arg = format!("%{}%", sn);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(status) = &r.status {
        let sql = " AND status=";

        q.push(sql).push_bind(status);
        qc.push(sql).push_bind(status);
    }

    if let Some(delivery_id) = &r.delivery_id {
        let sql = " AND delivery_id ILIKE ";
        let arg = format!("%{}%", delivery_id);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(dr) = &r.date_range {
        q.push(" AND ( dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
        qc.push(" AND ( dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
    }

    if let Some(dr) = &r.cancel_date_range {
        q.push(" AND ( cancel_until_dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
        qc.push(" AND ( cancel_until_dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
    }

    if let Some(dr) = &r.confirm_date_range {
        q.push(" AND ( confirm_until_dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
        qc.push(" AND ( confirm_until_dateline BETWEEN ")
            .push_bind(&dr.start)
            .push(" AND ")
            .push_bind(&dr.end)
            .push(")");
    }

    if let Some(ar) = &r.amount_range {
        q.push(" AND ( amount BETWEEN ")
            .push_bind(&ar.min)
            .push(" AND ")
            .push_bind(&ar.max)
            .push(")");
        qc.push(" AND ( amount BETWEEN ")
            .push_bind(&ar.min)
            .push(" AND ")
            .push_bind(&ar.max)
            .push(")");
    }

    if let Some(consignee) = &r.consignee {
        let sql = " AND address ->> 'consignee' ILIKE ";
        let arg = format!("%{}%", consignee);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(phone) = &r.phone {
        let sql = " AND address ->> 'phone' ILIKE ";
        let arg = format!("%{}%", phone);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(address) = &r.address {
        let sql = " AND address ->> 'address' ILIKE ";
        let arg = format!("%{}%", address);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
        .push_bind(r.paginate.page_size())
        .push(" OFFSET ")
        .push_bind(r.paginate.offset());

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(Paginate::quick(&r.paginate, &count, data))
}

#[cfg(test)]
mod test {

    use crate::{
        db,
        model::{self, U32},
        utils,
    };
    // const TEST_GOODS_ID: &str = "ckmivtsdrfagh6slh3tg";
    const TEST_USER_ID: &str = "cj7kfuel6bcqn8bicrng";
    const TEST_CART_ID: &str = "cksauc4drfao8bsjqhsg";
    const TEST_ORDER_ID: &str = "ckt123cdrfal3mj04d30";

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_db_create_order() {
        let dateline = chrono::Local::now();
        let cancel_until_dateline = dateline + chrono::Duration::minutes(15); // 15分钟
        let confirm_until_dateline = dateline + chrono::Duration::days(7); // 7天
        let order = model::Order {
            user_id: TEST_USER_ID.into(),
            cart_id: TEST_CART_ID.into(),
            sn: utils::id::new(),
            status: model::OrderStatus::PendingPay,
            amount: U32::from_yuan(10),
            total_num: U32::from(1),
            freight: U32::from(0),
            address: sqlx::types::Json::from(model::AddressDetail {
                consignee: format!("张三"),
                phone: format!("13800138000"),
                address: format!("广园南路1024-512号"),
                province: format!("广东"),
                city: format!("广州"),
                county: format!("天河区"),
                post_code: format!("512345"),
            }),
            delivery_id: "SF0987654321".into(),
            dateline,
            cancel_until_dateline,
            confirm_until_dateline,
            ..Default::default()
        };
        let conn = get_conn().await;
        let id = super::create(&conn, &order).await.unwrap();
        assert!(!id.is_empty());
        println!("id: {}", id);
    }
    #[tokio::test]
    async fn test_db_update_order_amount() {
        let r = model::EditOrderAmountRequest {
            id: TEST_ORDER_ID.into(),
            amount: U32::from_yuan(20),
        };
        let conn = get_conn().await;
        let aff = super::edit_amount(&conn, &r).await.unwrap();
        assert!(aff > 0);
    }
    #[tokio::test]
    async fn test_db_update_order_address() {
        let r = model::EditOrderAddressRequest {
            id: TEST_ORDER_ID.into(),
            address: sqlx::types::Json::from(model::AddressDetail {
                consignee: format!("里斯"),
                phone: format!("13911139111"),
                address: format!("机场东路7890号"),
                province: format!("广东"),
                city: format!("深圳"),
                county: format!("宝安区"),
                post_code: format!("598765"),
            }),
        };
        let conn = get_conn().await;
        let aff = super::edit_address(&conn, &r).await.unwrap();
        assert!(aff > 0);
    }
    #[tokio::test]
    async fn test_db_update_order_status() {
        let r = model::EditOrderStatusRequest {
            id: TEST_ORDER_ID.into(),
            // status: model::OrderStatus::Paied,
            status: model::OrderStatus::UserConfirmedDone,
            pre_status: Some(model::OrderStatus::Paied),
        };
        let conn = get_conn().await;
        let aff = super::edit_status(&conn, &r).await.unwrap();
        assert!(aff > 0);
    }
    #[tokio::test]
    async fn test_db_del_or_restore_order() {
        let conn = get_conn().await;
        let aff = super::del_or_restore(&conn, TEST_ORDER_ID, true, Some(TEST_USER_ID.into()))
            .await
            .unwrap();
        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_db_find_order() {
        let r = model::FindOrderRequest {
            // by: model::FindOrderBy::ID(TEST_ORDER_ID.into()),
            by: model::FindOrderBy::SN("ckt123cdrfal3mj04d2g".into()),
            user_id: Some(TEST_USER_ID.into()),
            is_del: Some(false),
        };
        let conn = get_conn().await;
        let order = super::find(&conn, &r).await.unwrap();
        assert!(order.is_some());
        println!("{:?}", order);
    }
    #[tokio::test]
    async fn test_db_list_order() {
        let now = chrono::Local::now();
        let date_range: Option<model::DatetimeRange> = Some(model::DatetimeRange {
            start: now - chrono::Duration::days(365),
            end: now,
        });
        let cancel_date_range: Option<model::DatetimeRange> = Some(model::DatetimeRange {
            start: now - chrono::Duration::days(365),
            end: now + chrono::Duration::days(365),
        });
        let confirm_date_range: Option<model::DatetimeRange> = Some(model::DatetimeRange {
            start: now - chrono::Duration::days(365),
            end: now + chrono::Duration::days(365),
        });
        let amount_range: Option<model::U32Range> = Some(model::U32Range {
            min: U32::from_yuan(1),
            max: U32::from_yuan(200),
        });

        let r = model::ListOrderRequest {
            paginate: db::PaginateRequest {
                page: 0,
                page_size: 3,
            },
            user_id: Some(TEST_USER_ID.into()),
            consignee: Some("张".into()),
            phone: Some("138".into()),
            address: Some("广园".into()),
            is_del: Some(false),
            sn: Some("kt175".into()),
            status: Some(model::OrderStatus::PendingPay),
            delivery_id: Some("SF".into()),
            date_range,
            cancel_date_range,
            confirm_date_range,
            amount_range,
        };
        let conn = get_conn().await;
        let p = super::list(&conn, &r).await.unwrap();
        println!("{}, {}", p.total, p.total_page);

        for order in p.data.iter() {
            println!("{:?}", order);
        }
    }
}
