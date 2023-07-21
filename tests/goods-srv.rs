use mall::pb::{self, goods_service_client::GoodsServiceClient};
use tonic::transport::Channel;

async fn get_client() -> GoodsServiceClient<Channel> {
    let dst = std::env::var("MALL_SERVER_ADDR").unwrap_or("http://127.0.0.1:9527".to_string());
    GoodsServiceClient::connect(dst).await.unwrap()
}

// #region 品牌测试用例

#[tokio::test]
async fn test_create_brand() {
    let mut cli = get_client().await;
    let resp = cli
        .create_brand(tonic::Request::new(mall::pb::Brand {
            name: "AXUM中文网".into(),
            logo: "https://file.axum.rs/asset/logo.png".into(),
            ..Default::default()
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.value > 0);
}

#[tokio::test]
async fn test_batch_create_brand() {
    let mut cli = get_client().await;
    for i in [
        (
            "MySQL",
            "https://file.axum.rs/icons/mysql/mysql-original.svg",
        ),
        ("RabbitMQ", "https://file.axum.rs/icons/rabbitmq.png"),
        (
            "PostgreSQL",
            "https://file.axum.rs/icons/postgresql/postgresql-original.svg",
        ),
        (
            "React",
            "https://file.axum.rs/icons/react/react-original.svg",
        ),
        ("Git", "https://file.axum.rs/icons/git/git-plain.svg"),
        ("Rust", "https://file.axum.rs/icons/rust/rust-plain.svg"),
        ("Go", "https://file.axum.rs/icons/go/go-original.svg"),
    ] {
        let resp = cli
            .create_brand(tonic::Request::new(mall::pb::Brand {
                name: i.0.into(),
                logo: i.1.into(),
                ..Default::default()
            }))
            .await
            .unwrap()
            .into_inner();
        assert!(resp.value > 0);
    }
}

#[tokio::test]
async fn test_find_brand() {
    let mut cli = get_client().await;
    let resp = cli
        .find_brand(tonic::Request::new(mall::pb::FindBrandRequest {
            is_del: None,
            by: Some(pb::find_brand_request::By::Name("Rust".into())),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.brand.is_some_and(|b| b.name == "Rust".to_string()));
}

#[tokio::test]
async fn test_list_brand() {
    let mut cli = get_client().await;
    let resp = cli
        .list_brand(tonic::Request::new(pb::ListBrandRequest {
            paginate: Some(pb::PaginateRequest {
                page: Some(0),
                page_size: Some(2),
            }),
            name: Some("Q".into()),
            is_del: Some(false),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.paginate.is_some_and(|p| p.total_page > 0));
    assert!(resp.brands.len() > 0);
    for b in resp.brands.iter() {
        println!("{} {}", b.name, b.logo);
    }
}

#[tokio::test]
async fn test_edit_brand() {
    let mut cli = get_client().await;

    // 查找出欲修改的品牌，避免手动构造
    let resp = cli
        .find_brand(tonic::Request::new(pb::FindBrandRequest {
            is_del: Some(false),
            by: Some(pb::find_brand_request::By::Name("Go".into())),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.brand.is_some());
    let brand = resp.brand.unwrap();

    // 修改
    let resp = cli
        .edit_brand(tonic::Request::new(pb::Brand {
            name: "Go语言".into(),
            // name: "Rust".into(), // 已经存在了“Rust”，所以预期要报错：品牌已存在
            logo: "https://file.axum.rs/icons/go/go-plain.svg".into(),
            ..brand
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1);
}

#[tokio::test]
async fn test_brand_del_or_restore() {
    let mut cli = get_client().await;
    let id = 7087988042216116224 as u64; // “AXUM中文网”的ID，请根据你的情况进行修改
    let is_del = false; // true: 删除；false：还原
    let resp = cli
        .delete_or_restore_brand(tonic::Request::new(pb::DeleteOrRestoreRequest {
            id,
            is_del,
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1);
}

#[tokio::test]
async fn test_brand_exist() {
    let mut cli = get_client().await;
    let resp = cli
        .brand_exists(tonic::Request::new(pb::BrandExistsRequest {
            name: "AXUM中文网".into(),
            id: None,
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.value);
}

// #endregion 品牌测试用例
