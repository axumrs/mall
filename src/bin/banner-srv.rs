use mall::{
    config::ServerConfig, etcd, pb::banner_service_server::BannerServiceServer, srv::banner_srv,
};

#[tokio::main]
async fn main() {
    let cli = etcd::conn().await.unwrap();
    let cfg: Option<ServerConfig> = etcd::get(&cli, "/banner-srv").await.unwrap();
    let cfg = cfg.unwrap_or_default();

    println!("{:?}", cfg);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let srv = banner_srv::Banner::new(pool);

    tonic::transport::Server::builder()
        .add_service(BannerServiceServer::new(srv))
        .serve((&cfg.addr).parse().unwrap())
        .await
        .unwrap();
}
