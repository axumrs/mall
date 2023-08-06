use mall::{
    config::ServerConfig, etcd, pb::goods_service_server::GoodsServiceServer, srv::goods_srv,
};

#[tokio::main]
async fn main() {
    let cli = etcd::conn().await.unwrap();
    let cfg: Option<ServerConfig> = etcd::get(&cli, "/goods-srv").await.unwrap();
    let cfg = cfg.unwrap_or_default();

    println!("{:?}", cfg);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let srv = goods_srv::Goods::new(pool);

    tonic::transport::Server::builder()
        .add_service(GoodsServiceServer::new(srv))
        .serve((&cfg.addr).parse().unwrap())
        .await
        .unwrap();
}
