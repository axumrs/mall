#[derive(Debug)]
pub enum Kind {
    Database,
    Etcd,
    Serde,
    Bcrypt,
    NotFound,
    IncorrectAuth,
    InvalidArgument,
    Base16ct,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(kind: Kind, message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }
    pub fn with_cause(kind: Kind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }
    pub fn from_string(kind: Kind, message: String) -> Self {
        Self::new(kind, message, None)
    }
    pub fn from_str(kind: Kind, msg: &str) -> Self {
        Self::from_string(kind, msg.to_string())
    }
    pub fn not_found_with(msg: &str) -> Self {
        Self::from_str(Kind::NotFound, msg)
    }
    pub fn not_found() -> Self {
        Self::not_found_with("不存在的记录")
    }
    pub fn incorrect_auth_with(msg: &str) -> Self {
        Self::from_str(Kind::IncorrectAuth, msg)
    }
    pub fn incorrect_auth() -> Self {
        Self::incorrect_auth_with("用户名或密码错误")
    }
    pub fn invalid_argument(msg: &str) -> Self {
        Self::from_str(Kind::InvalidArgument, msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::with_cause(Kind::Database, Box::new(e))
    }
}

impl From<etcd_rs::Error> for Error {
    fn from(e: etcd_rs::Error) -> Self {
        Self::with_cause(Kind::Etcd, Box::new(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::with_cause(Kind::Serde, Box::new(e))
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(e: bcrypt::BcryptError) -> Self {
        Self::with_cause(Kind::Bcrypt, Box::new(e))
    }
}

impl From<base16ct::Error> for Error {
    fn from(e: base16ct::Error) -> Self {
        Self::from_string(Kind::Base16ct, e.to_string())
    }
}
