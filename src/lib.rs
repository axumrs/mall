pub mod config;
pub mod db;
mod errs;
pub mod etcd;
pub mod model;
pub mod pb;
pub mod srv;
pub mod types;
pub mod utils;
pub mod web_api;

pub use errs::Error;
pub use errs::Kind as ErrorKind;

pub type Result<T> = std::result::Result<T, crate::Error>;
