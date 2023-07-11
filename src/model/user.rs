use serde::{Deserialize, Serialize};

use crate::{db, pb, utils::dt};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[repr(u8)]
pub enum UserStatus {
    /// 待激活
    #[default]
    Pending = 0,
    /// 正常，已激活
    Actived = 1,
    /// 被冻结
    Freezed = 2,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub status: UserStatus,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

impl From<pb::UserStatus> for UserStatus {
    fn from(us: pb::UserStatus) -> Self {
        match us {
            pb::UserStatus::Actived => Self::Actived,
            pb::UserStatus::Pending => Self::Pending,
            pb::UserStatus::Freezed => Self::Freezed,
        }
    }
}

impl Into<pb::UserStatus> for UserStatus {
    fn into(self) -> pb::UserStatus {
        match self {
            Self::Actived => pb::UserStatus::Actived,
            Self::Freezed => pb::UserStatus::Freezed,
            Self::Pending => pb::UserStatus::Pending,
        }
    }
}

impl From<pb::User> for User {
    fn from(u: pb::User) -> Self {
        let status = UserStatus::from(u.status());
        Self {
            id: u.id,
            email: u.email,
            password: u.password,
            nickname: u.nickname,
            status,
            dateline: dt::prost2chrono(&u.dateline),
            is_del: u.is_del,
        }
    }
}

impl Into<pb::User> for User {
    fn into(self) -> pb::User {
        let status: pb::UserStatus = self.status.into();

        pb::User {
            id: self.id,
            email: self.email,
            password: self.password,
            nickname: self.nickname,
            status: status.into(),
            dateline: dt::chrono2prost(&self.dateline),
            is_del: self.is_del,
        }
    }
}

pub enum UserFindBy {
    ID(u64),
    Email(String),
}

pub struct UserFindRequest {
    pub by: UserFindBy,
    pub is_del: Option<bool>,
    pub status: Option<UserStatus>,
}

impl From<pb::FindUserRequest> for UserFindRequest {
    fn from(r: pb::FindUserRequest) -> Self {
        let by = r.by.unwrap();
        let by = match by {
            pb::find_user_request::By::Email(ref email) => UserFindBy::Email(email.to_owned()),
            pb::find_user_request::By::Id(ref id) => UserFindBy::ID(id.to_owned()),
        };
        let status = match r.status {
            Some(i) => Some(UserStatus::from(pb::UserStatus::from_i32(i).unwrap())),
            None => None,
        };
        Self {
            by,
            is_del: r.is_del,
            status,
        }
    }
}

pub struct UserListRequest {
    pub paginate: db::paginate::PaginateRequest,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<UserStatus>,
    pub is_del: Option<bool>,
    pub date_range: Option<super::DatetimeRange>,
}

impl From<pb::ListUserRequest> for UserListRequest {
    fn from(r: pb::ListUserRequest) -> Self {
        let status = match r.status {
            Some(s) => Some(UserStatus::from(pb::UserStatus::from_i32(s).unwrap())),
            None => None,
        };
        let date_range = match r.date_range {
            Some(dr) => Some(super::DatetimeRange::from(dr)),
            None => None,
        };
        Self {
            paginate: db::paginate::PaginateRequest::from(r.paginate.unwrap()),
            email: r.email,
            nickname: r.nickname,
            status,
            is_del: r.is_del,
            date_range,
        }
    }
}

impl Into<pb::ListUserRequest> for UserListRequest {
    fn into(self) -> pb::ListUserRequest {
        let date_range = match self.date_range {
            Some(dr) => Some(dr.into()),
            None => None,
        };
        let status = match self.status {
            Some(s) => {
                let us: pb::UserStatus = s.into();
                Some(us.into())
            }
            None => None,
        };
        pb::ListUserRequest {
            paginate: Some(self.paginate.into()),
            email: self.email,
            nickname: self.nickname,
            status,
            is_del: self.is_del,
            date_range,
        }
    }
}
