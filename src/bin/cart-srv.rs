use mall::{config::ServerConfig, etcd, pb::cart_service_server::CartServiceServer, srv::cart_srv};

#[tokio::main]
async fn main() {
    let cli = etcd::conn().await.unwrap();
    let cfg: Option<ServerConfig> = etcd::get(&cli, "/cart-srv").await.unwrap();
    let cfg = cfg.unwrap_or_default();

    println!("{:?}", cfg);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let srv = cart_srv::Cart::new(pool);

    tonic::transport::Server::builder()
        .add_service(CartServiceServer::new(srv))
        .serve((&cfg.addr).parse().unwrap())
        .await
        .unwrap();
}
