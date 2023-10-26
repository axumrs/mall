use mall::{
    config::ServerConfig, etcd, pb::order_service_server::OrderServiceServer, srv::order_srv,
};

#[tokio::main]
async fn main() {
    let cli = etcd::conn().await.unwrap();
    let cfg: Option<ServerConfig> = etcd::get(&cli, "/order-srv").await.unwrap();
    let cfg = cfg.unwrap_or_default();

    println!("{:?}", cfg);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let srv = order_srv::Order::new(pool);

    tonic::transport::Server::builder()
        .add_service(OrderServiceServer::new(srv))
        .serve((&cfg.addr).parse().unwrap())
        .await
        .unwrap();
}
