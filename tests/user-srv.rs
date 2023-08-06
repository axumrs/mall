use mall::{
    pb::{self, user_service_client::UserServiceClient},
    utils::dt,
};
use tonic::transport::Channel;

// 运行单个测试函数：cargo test --test user-srv 函数名
// 比如运行 test_create_user 测试函数：cargo test --test user-srv test_create_user

async fn get_client() -> UserServiceClient<Channel> {
    let dst = std::env::var("MALL_SERVER_ADDR").unwrap_or("http://127.0.0.1:9527".to_string());
    UserServiceClient::connect(dst).await.unwrap()
}

#[tokio::test]
async fn test_create_user() {
    let mut cli = get_client().await;
    let dateline = dt::chrono2prost(&chrono::Local::now());
    let resp = cli
        .create_user(tonic::Request::new(pb::User {
            email: "team@axum.rs".into(),
            nickname: "AXUM中文网".into(),
            password: "axum.rs".into(),
            status: pb::UserStatus::Actived.into(),
            dateline,
            ..Default::default()
        }))
        .await
        .unwrap();
    assert!(!resp.into_inner().value.is_empty());
}

#[tokio::test]
async fn test_batch_create_user() {
    let mut cli = get_client().await;
    for i in [
        ("zs@foo.bar", "张三"),
        ("ls@foo.bar", "李四"),
        ("ww@foo.bar", "王五"),
        ("zl@foo.bar", "赵六"),
        ("qq@foo.bar", "钱七"),
    ] {
        let dateline = dt::chrono2prost(&chrono::Local::now());
        let resp = cli
            .create_user(tonic::Request::new(pb::User {
                email: i.0.into(),
                nickname: i.1.into(),
                password: "axum.rs".into(),
                status: pb::UserStatus::Actived.into(),
                dateline,
                ..Default::default()
            }))
            .await
            .unwrap();
        assert!(!resp.into_inner().value.is_empty());
    }
}

#[tokio::test]
async fn test_user_exists() {
    let mut cli = get_client().await;
    let resp = cli
        .user_exists(tonic::Request::new(pb::UserExistsRequest {
            email: "team@axum.rs".into(),
            nickname: None,
            id: None,
        }))
        .await
        .unwrap();
    assert!(resp.into_inner().is_exists)
}

#[tokio::test]
async fn test_find_user() {
    let mut cli = get_client().await;

    let resp = cli
        .find_user(tonic::Request::new(pb::FindUserRequest {
            status: None,
            is_del: None,
            by: Some(pb::find_user_request::By::Email("team@axum.rs".into())),
        }))
        .await
        .unwrap();

    assert!(resp
        .into_inner()
        .user
        .is_some_and(|u| u.email.eq("team@axum.rs".into())))
}

#[tokio::test]
async fn test_list_user() {
    let mut cli = get_client().await;

    // let dr_end = chrono::Local::now();
    // let dr_start = dr_end - chrono::Duration::days(30);
    // let date_range = Some(pb::DateRange {
    //     start: dt::chrono2prost(&dr_start),
    //     end: dt::chrono2prost(&dr_end),
    // });
    let date_range = None;

    let resp = cli
        .list_user(tonic::Request::new(pb::ListUserRequest {
            paginate: Some(pb::PaginateRequest {
                page: Some(0),
                page_size: Some(3),
            }),
            email: Some("axum.rs".into()),
            // email: None,
            // nickname: Some("AXUM".into()),
            nickname: None,
            // status: Some(pb::UserStatus::Actived.into()),
            status: None,
            // is_del: Some(false),
            is_del: None,
            date_range,
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.paginate.is_some_and(|p| p.total_page > 0));
    assert!(resp.users.len() > 0);
    for u in resp.users.iter() {
        println!("{} {}", u.email, u.nickname);
    }
}

#[tokio::test]
async fn test_edit_user() {
    let mut cli = get_client().await;
    // 查找用于修改的用户，省去手动构造
    let resp = cli
        .find_user(tonic::Request::new(pb::FindUserRequest {
            status: None,
            is_del: None,
            by: Some(pb::find_user_request::By::Email("zs@foo.bar".into())),
        }))
        .await
        .unwrap()
        .into_inner();
    let resp = cli
        .edit_user(tonic::Request::new(pb::User {
            nickname: "张三1".into(), // 将昵称修改为“张三1”
            // nickname: "李四".into(), // 其他用户已经使用了“李四”，所以预期要报错：“昵称已存在”
            ..resp.user.unwrap()
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1);
}

#[tokio::test]
async fn test_change_user_password() {
    let mut cli = get_client().await;
    let id = "cj7kfuel6bcqn8bicrng".to_string(); // 用户“张三”的ID，根据你的情况进行修改

    // // 正确的当前密码（用户自己修改密码）
    // let current_password: Option<String> = Some("axum.rs".into());
    //  // 错误的当前密码（用户自己修改密码）
    // let current_password: Option<String> = Some("abcdefg".into());
    //  // 不提供当前密码（管理员后台修改密码）
    let current_password: Option<String> = None;
    let resp = cli
        .change_user_password(tonic::Request::new(pb::ChangeUserPasswordRequest {
            id,
            password: "axum.rs".into(),
            current_password,
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1)
}

#[tokio::test]
async fn test_change_user_status() {
    let mut cli = get_client().await;
    let id = "cj7kfuel6bcqn8bicrng".to_string(); // 用户“张三”的ID，根据你的情况进行修改
    let resp = cli
        .change_user_status(tonic::Request::new(pb::ChangeUserStatusRequest {
            id,
            status: pb::UserStatus::Freezed.into(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1)
}
#[tokio::test]
async fn test_user_del_or_restore() {
    let mut cli = get_client().await;
    let id = "cj7kfuel6bcqn8bicrng".to_string(); // 用户“张三”的ID，根据你的情况进行修改
    let is_del = true; // true：删除；false：恢复
    let resp = cli
        .delete_or_restore_user(tonic::Request::new(pb::DeleteOrRestoreRequest {
            id,
            is_del,
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(resp.rows == 1)
}
