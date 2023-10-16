use crate::{model, utils, Error};

use super::Paginate;

pub async fn exists<'a>(
    e: impl sqlx::PgExecutor<'a>,
    r: &'a model::GoodsExistsRequest,
) -> Result<bool, sqlx::Error> {
    let mut q = sqlx::QueryBuilder::new(" SELECT COUNT(*) FROM goods WHERE 1=1");
    match &r.by {
        model::GoodsExistsBy::Name(name) => q.push(" AND name=").push_bind(name),
        model::GoodsExistsBy::SN(sn) => q.push(" AND sn=").push_bind(sn),
    };

    if let Some(id) = &r.id {
        q.push(" AND id <>").push_bind(id);
    }

    let count: (i64,) = q.build_query_as().fetch_one(e).await?;

    Ok(count.0 > 0)
}

pub async fn create<'a>(
    e: impl sqlx::PgExecutor<'a>,
    g: &model::Goods,
) -> Result<String, sqlx::Error> {
    let id = utils::id::new();

    let cover = if g.cover.is_empty() {
        if g.images.is_empty() {
            "".to_string()
        } else {
            g.images[0].clone()
        }
    } else {
        g.cover.clone()
    };

    sqlx::query(r#"INSERT INTO goods (id, category_id, "name", sn, is_sale, ship_fee, is_new, is_hot, hit, sold_num, fav_num, origin_price, price, brief, cover, images, description, dateline, is_del) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)"#)
    .bind(&id)
    .bind(&g.category_id)
    .bind(&g.name)
    .bind(&g.sn)
    .bind(&g.is_sale)
    .bind(&g.ship_fee.to_cent())
    .bind(&g.is_new)
    .bind(&g.is_hot)
    .bind(&g.hit)
    .bind(&g.sold_num)
    .bind(&g.fav_num)
    .bind(&g.origin_price.to_cent())
    .bind(&g.price.to_cent())
    .bind(&g.brief)
    .bind(&cover)
    .bind(&g.images)
    .bind(&g.description)
    .bind(&g.dateline)
    .bind(&g.is_del)
    .execute(e).await?;
    Ok(id)
}

pub async fn edit<'a>(
    e: impl sqlx::PgExecutor<'a>,
    g: &'a model::Goods,
) -> Result<u64, sqlx::Error> {
    let aff = sqlx::query(r#"UPDATE goods SET category_id=$1, "name"=$2, sn=$3, is_sale=$4, ship_fee=$5, is_new=$6, is_hot=$7, hit=$8, sold_num=$9, fav_num=$10, origin_price=$11, price=$12, brief=$13, cover=$14, images=$15, description=$16 WHERE id=$17"#)
    .bind(&g.category_id)
    .bind(&g.name)
    .bind(&g.sn)
    .bind(&g.is_sale)
    .bind(&g.ship_fee.to_cent())
    .bind(&g.is_new)
    .bind(&g.is_hot)
    .bind(&g.hit)
    .bind(&g.sold_num)
    .bind(&g.fav_num)
    .bind(&g.origin_price.to_cent())
    .bind(&g.price.to_cent())
    .bind(&g.brief)
    .bind(&g.cover)
    .bind(&g.images)
    .bind(&g.description)
    .bind(&g.id)
    .execute(e).await?.rows_affected();

    Ok(aff)
}

pub async fn del_or_restore(conn: &sqlx::PgPool, id: String, is_del: bool) -> crate::Result<u64> {
    super::del_or_restore(conn, "goods", id, is_del).await
}

pub async fn find(
    conn: &sqlx::PgPool,
    r: &model::FindGoodsRequest,
) -> crate::Result<Option<model::Goods>> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, category_id, "name", sn, is_sale, ship_fee, is_new, is_hot, hit, sold_num, fav_num, origin_price, price, brief, cover, images, description, dateline, is_del FROM goods WHERE 1=1"#,
    );
    match &r.by {
        model::FindGoodsBy::ID(id) => q.push(" AND id=").push_bind(id),
        model::FindGoodsBy::SN(sn) => q.push(" AND sn=").push_bind(sn),
    };

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    let g = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(g)
}

pub async fn list(
    conn: &sqlx::PgPool,
    r: &model::ListGoodsRequest,
) -> crate::Result<Paginate<model::Goods>> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, category_id, "name", sn, is_sale, ship_fee, is_new, is_hot, hit, sold_num, fav_num, origin_price, price, brief, cover, images, description, dateline, is_del FROM goods WHERE 1=1"#,
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM goods WHERE 1=1");

    if let Some(name) = &r.name {
        let sql = " AND name ILIKE ";
        let arg = format!("%{}%", name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(is_del) = &r.is_del {
        q.push(" AND is_del=").push_bind(is_del);
        qc.push(" AND is_del=").push_bind(is_del);
    }

    if let Some(sn) = &r.sn {
        let sql = " AND sn ILIKE ";
        let arg = format!("%{}%", sn);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(category_id) = &r.category_id {
        q.push(" AND category_id=").push_bind(category_id);
        qc.push(" AND category_id=").push_bind(category_id);
    }

    if let Some(ship_fee_range) = &r.ship_fee_range {
        q.push(" AND (ship_fee BETWEEN ")
            .push_bind(ship_fee_range.min.to_cent());
        q.push(" AND ")
            .push_bind(ship_fee_range.max.to_cent())
            .push(")");

        qc.push(" AND (ship_fee BETWEEN ")
            .push_bind(ship_fee_range.min.to_cent());
        qc.push(" AND ")
            .push_bind(ship_fee_range.max.to_cent())
            .push(")");
    }

    if let Some(origin_price_range) = &r.origin_price_range {
        q.push(" AND (origin_price BETWEEN ")
            .push_bind(origin_price_range.min.to_cent());
        q.push(" AND ")
            .push_bind(origin_price_range.max.to_cent())
            .push(")");

        qc.push(" AND (origin_price BETWEEN ")
            .push_bind(origin_price_range.min.to_cent());
        qc.push(" AND ")
            .push_bind(origin_price_range.max.to_cent())
            .push(")");
    }

    if let Some(price_range) = &r.price_range {
        q.push(" AND (price BETWEEN ")
            .push_bind(price_range.min.to_cent());
        q.push(" AND ")
            .push_bind(price_range.max.to_cent())
            .push(")");

        qc.push(" AND (price BETWEEN ")
            .push_bind(price_range.min.to_cent());
        qc.push(" AND ")
            .push_bind(price_range.max.to_cent())
            .push(")");
    }

    if let Some(date_range) = &r.date_range {
        q.push(" AND (dateline BETWEEN ")
            .push_bind(date_range.start);
        q.push(" AND ").push_bind(date_range.end).push(")");

        qc.push(" AND (dateline BETWEEN ")
            .push_bind(date_range.start);
        qc.push(" AND ").push_bind(date_range.end).push(")");
    }

    if let Some(is_sale) = &r.is_sale {
        q.push(" AND is_sale=").push_bind(is_sale);
        qc.push(" AND is_sale=").push_bind(is_sale);
    }

    if let Some(order) = &r.primary_order {
        match order {
            model::ListGoodsOrder::ByHit => q.push(" ORDER BY hit DESC"),
            model::ListGoodsOrder::ById => q.push(" ORDER BY id DESC"),
            model::ListGoodsOrder::ByIsNew => q.push(" ORDER BY is_new DESC"),
            model::ListGoodsOrder::ByIsHot => q.push(" ORDER BY is_hot DESC"),
            model::ListGoodsOrder::BySoldNum => q.push(" ORDER BY sold_num DESC"),
            model::ListGoodsOrder::ByShipFee => q.push(" ORDER BY ship_fee DESC"),
            model::ListGoodsOrder::ByOriginPrice => q.push(" ORDER BY origin_price DESC"),
            model::ListGoodsOrder::ByPrice => q.push(" ORDER BY price DESC"),
            model::ListGoodsOrder::ByIsSale => q.push(" ORDER BY is_sale DESC"),
        };
        if let Some(s_order) = &r.secondary_order {
            match s_order {
                model::ListGoodsOrder::ByHit => q.push(", hit DESC"),
                model::ListGoodsOrder::ById => q.push(", id DESC"),
                model::ListGoodsOrder::ByIsNew => q.push(", is_new DESC"),
                model::ListGoodsOrder::ByIsHot => q.push(", is_hot DESC"),
                model::ListGoodsOrder::BySoldNum => q.push(", sold_num DESC"),
                model::ListGoodsOrder::ByShipFee => q.push(", ship_fee DESC"),
                model::ListGoodsOrder::ByOriginPrice => q.push(", origin_price DESC"),
                model::ListGoodsOrder::ByPrice => q.push(", price DESC"),
                model::ListGoodsOrder::ByIsSale => q.push(", is_sale DESC"),
            };
        }
    } else {
        q.push(" ORDER BY id DESC");
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
        model::{self, Money},
        utils,
    };

    async fn get_conn() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://mall:mall@127.0.0.1:5432/mall")
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_db_create_goods() {
        let conn = get_conn().await;
        let g = model::Goods {
            category_id: "cji1llcdrfap1bhmp750".to_string(),
            name: "商品".to_string(),
            sn: utils::id::new(),
            is_sale: true,
            ship_fee: Money::new(1),
            origin_price: Money::new(5),
            price: Money::new(3),
            brief: "简介".to_string(),
            cover: "".to_string(),
            images: vec!["https://file.axum.rs/asset/logo.png".to_string()],
            description: "详情".to_string(),
            dateline: chrono::Local::now(),
            ..Default::default()
        };
        let id = super::create(&conn, &g).await.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn test_db_batch_create_goods() {
        const NUMS: usize = 5;
        let mut ids = Vec::with_capacity(NUMS);

        let conn = get_conn().await;

        for i in 0..NUMS {
            let sn = utils::id::new();
            let g = model::Goods {
                category_id: "cji1llcdrfap1bhmp6v0".to_string(),
                name: format!("商品-{}", i),
                sn,
                is_sale: i % 2 == 0,
                ship_fee: Money::new(1) * Money::new(i as i32),
                origin_price: Money::new(5) + Money::new(i as i32),
                price: Money::new(3) + Money::new(i as i32),
                brief: format!("简介-{}", i),
                cover: "".to_string(),
                images: vec!["https://file.axum.rs/asset/logo.png".to_string()],
                description: format!("详情-{}", i),
                dateline: chrono::Local::now(),
                ..Default::default()
            };
            let id = super::create(&conn, &g).await.unwrap();
            ids.push(id);
        }

        assert!(ids.len() == NUMS);
        assert!(ids.iter().all(|id| !id.is_empty()));
    }

    #[tokio::test]
    async fn test_db_edit_goods() {
        let conn = get_conn().await;
        let g = model::Goods {
            id: "ckmivt4drfagiufo7ohg".to_string(),
            category_id: "cji1llcdrfap1bhmp750".to_string(),
            name: "商品-EDITED".to_string(),
            sn: utils::id::new(),
            is_sale: true,
            ship_fee: Money::new(11),
            origin_price: Money::new(51),
            price: Money::new(31),
            brief: "简介-EDITOED".to_string(),
            cover: "https://file.axum.rs/asset/logo.png-1".to_string(),
            images: vec!["https://file.axum.rs/asset/logo.png-2".to_string()],
            description: "详情-EDITED".to_string(),
            dateline: chrono::Local::now(),
            ..Default::default()
        };
        let aff = super::edit(&conn, &g).await.unwrap();
        assert!(aff == 1);
    }

    #[tokio::test]
    async fn test_db_delete_or_restore_goods() {
        let conn = get_conn().await;
        let aff = super::del_or_restore(&conn, "ckmivt4drfagiufo7ohg".to_string(), false)
            .await
            .unwrap();
        assert!(aff == 1);
    }

    #[tokio::test]
    async fn test_db_find_goods() {
        let r = model::FindGoodsRequest {
            // by: model::FindGoodsBy::ID("ckmivt4drfagiufo7ohg".to_string()),
            by: model::FindGoodsBy::SN("ckmj2vkdrfahphfprt00".to_string()),
            is_del: None,
        };
        let conn = get_conn().await;
        let g = super::find(&conn, &r).await.unwrap();
        assert!(g.is_some());
    }

    #[tokio::test]
    async fn test_db_list_goods() {
        let r = model::ListGoodsRequest {
            paginate: db::PaginateRequest {
                page: 0,
                page_size: 3,
            },
            name: Some("商品".to_string()),
            is_del: Some(false),
            sn: Some("ckmivtsdrfagh6slh3v0".to_string()),
            category_id: Some("cji1llcdrfap1bhmp6v0".to_string()),
            ship_fee_range: Some(model::U32Range {
                min: 0.into(),
                max: 10.into(),
            }),
            origin_price_range: Some(model::U32Range {
                min: 6.into(),
                max: 8.into(),
            }),
            price_range: Some(model::U32Range {
                min: 5.into(),
                max: 7.into(),
            }),
            date_range: Some(model::DatetimeRange {
                start: chrono::Local::now() - chrono::Duration::days(7),
                end: chrono::Local::now(),
            }),
            is_sale: Some(true),
            primary_order: Some(model::ListGoodsOrder::ById),
            // primary_order: None,
            secondary_order: Some(model::ListGoodsOrder::ByHit),
        };
        let conn = get_conn().await;
        let p = super::list(&conn, &r).await.unwrap();

        for g in p.data.iter() {
            println!("{:?}", g);
        }

        println!("{}, {}", p.total, p.total_page);
    }
}
