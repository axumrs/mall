use std::env;

use etcd_rs::{Client, ClientConfig, Endpoint, KeyValueOp};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Result};

/// 通过指定参数连接到 etcd
pub async fn conn_with(endpoints: &[&str], name: &str, password: &str) -> Result<Client> {
    let endpoints: Vec<Endpoint> = endpoints.iter().map(|&s| s.into()).collect();
    let cfg = ClientConfig::new(endpoints).auth(name, password);
    Client::connect(cfg).await.map_err(Error::from)
}

/// 读取环境变量值连接到 etcd
/// - `MALL_ETCD_ENDPOINTS`：etcd的endpoints，多个地址使用英文逗号分隔。默认为 `http://localhost:2379`
/// - `MALL_ETCD_USER`：etcd 的用户名，默认为 `axum-rs`
/// - `MALL_ETCD_PASSWORD`：etcd 的密码，默认为 `axum.rs`
pub async fn conn() -> Result<Client> {
    let endpoints = env::var("MALL_ETCD_ENDPOINTS").unwrap_or("http://localhost:2379".to_string());
    let endpoints: Vec<&str> = endpoints.split(",").collect();
    let name = env::var("MALL_ETCD_USER").unwrap_or("axum-rs".to_string());
    let password = env::var("MALL_ETCD_PASSWORD").unwrap_or("axum.rs".to_string());
    conn_with(&endpoints, &name, &password).await
}

/// 从环境变量中读取 etcd 的前缀
/// - `MALL_ETCD_PREFIX`：前缀，默认为 `/axum.rs`
pub fn get_prefix() -> String {
    env::var("MALL_ETCD_PREFIX").unwrap_or("/axum.rs".to_string())
}

/// 生成 key
pub fn gen_key_with(prefix: &str, key: &str) -> String {
    format!("{}{}", prefix, key)
}

/// 生成 key，从环境变量中读取前缀
/// - `MALL_ETCD_PREFIX`：前缀，默认为 `/axum.rs`
pub fn gen_key(key: &str) -> String {
    let prefix = get_prefix();
    gen_key_with(&prefix, key)
}

/// 从 etcd 中获取值
pub async fn get_with<T: DeserializeOwned>(
    cli: &Client,
    prefix: &str,
    key: &str,
) -> Result<Option<T>> {
    let key = gen_key_with(prefix, key);
    let resp = cli.get(key.as_str()).await.map_err(Error::from)?;
    if let Some(kv) = resp.kvs.first() {
        let json_str = kv.value_str();
        let obj = serde_json::from_str::<T>(json_str).map_err(Error::from)?;
        return Ok(Some(obj));
    }
    Ok(None)
}

/// 从 etcd 中获取值，前缀从环境变量中读取
/// - `MALL_ETCD_PREFIX`：前缀，默认为 `/axum.rs`
pub async fn get<T: DeserializeOwned>(cli: &Client, key: &str) -> Result<Option<T>> {
    let prefix = get_prefix();
    get_with(cli, &prefix, key).await
}

/// 将数据写入 etcd
pub async fn put_with<T: Serialize>(
    cli: &Client,
    prefix: &str,
    key: &str,
    data: &T,
) -> Result<i64> {
    let key = gen_key_with(prefix, key);
    let json_str = serde_json::to_string(data).map_err(Error::from)?;
    let resp = cli
        .put((key.as_str(), json_str.as_str()))
        .await
        .map_err(Error::from)?;
    Ok(resp.header.revision())
}

/// 将数据写入 etcd，从环境变量中读取前缀
/// - `MALL_ETCD_PREFIX`：前缀，默认为 `/axum.rs`
pub async fn put<T: Serialize>(cli: &Client, key: &str, data: &T) -> Result<i64> {
    let prefix = get_prefix();
    put_with(cli, &prefix, key, data).await
}
