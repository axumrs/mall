use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfig {
    pub machine_id: i32,
    pub node_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub addr: String,
    pub db_dsn: String,
    pub node: NodeConfig,
}

impl std::default::Default for NodeConfig {
    fn default() -> Self {
        let machine_id = env::var("MALL_MACHINE_ID")
            .unwrap_or(1.to_string())
            .parse::<i32>()
            .unwrap_or(1);
        let node_id = env::var("MALL_NODE_ID")
            .unwrap_or(1.to_string())
            .parse::<i32>()
            .unwrap_or(1);

        Self {
            machine_id,
            node_id,
        }
    }
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        let addr = env::var("MALL_SERVER_ADDR").unwrap_or("0.0.0.0:9527".to_string());
        // .unwrap_or(get_available_addr().unwrap_or("0.0.0.0:9527".to_string()));

        let db_dsn =
            env::var("MALL_DB_DSN").unwrap_or("mysql://root:root@127.0.0.1:3306/mall".to_string());
        Self {
            addr,
            db_dsn,
            node: Default::default(),
        }
    }
}

/// 获取可用tcp端口
#[allow(unused)]
fn get_available_addr() -> Result<String, std::io::Error> {
    let addr = std::net::TcpListener::bind("0.0.0.0:0")?
        .local_addr()?
        .to_string();
    Ok(addr)
}
