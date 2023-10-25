pub mod banner_srv;
pub mod cart_srv;
pub mod goods_srv;
pub mod user_srv;

/// 将 SQLX 错误转换成 tonic 状态
fn sqlx_err_to_tonic_status(err: sqlx::Error) -> tonic::Status {
    tonic::Status::internal(err.to_string())
}

/// 将本 crate 错误转换成 tonic 状态
fn err_to_tonic_status(err: crate::Error) -> tonic::Status {
    tonic::Status::internal(err.message)
}
