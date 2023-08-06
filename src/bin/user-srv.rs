use mall::{config::ServerConfig, etcd, pb::user_service_server::UserServiceServer, srv::user_srv};

#[tokio::main]
async fn main() {
    let cli = etcd::conn().await.unwrap();
    let cfg: Option<ServerConfig> = etcd::get(&cli, "/user-srv").await.unwrap();
    let cfg = cfg.unwrap_or_default();

    println!("{:?}", cfg);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let srv = user_srv::User::new(pool);

    tonic::transport::Server::builder()
        .add_service(UserServiceServer::new(srv))
        .serve((&cfg.addr).parse().unwrap())
        .await
        .unwrap();
}
