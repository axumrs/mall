use std::sync::Arc;

use crate::{
    db, model,
    pb::{self, user_service_server::UserService, Id},
};

pub struct User {
    pool: Arc<sqlx::PgPool>,
}

impl User {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl UserService for User {
    async fn create_user(
        &self,
        request: tonic::Request<pb::User>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let u = request.into_inner();

        let exist = self
            .user_exists(tonic::Request::new(pb::UserExistsRequest {
                email: u.email.clone(),
                nickname: Some(u.nickname.clone()),
                id: None,
            }))
            .await?
            .into_inner();
        if exist.is_exists {
            return Err(tonic::Status::already_exists("email或昵称已存在"));
        }

        let m = model::User::from(u);

        let id = db::user::create(&self.pool, &m)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;

        let resp = Id { value: id };
        Ok(tonic::Response::new(resp))
    }
    async fn user_exists(
        &self,
        request: tonic::Request<pb::UserExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::UserExistsResponse>, tonic::Status> {
        let pb::UserExistsRequest {
            email,
            nickname,
            id,
        } = request.into_inner();
        let exists = db::user::exists(&self.pool, email, nickname, id)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        let resp = pb::UserExistsResponse { is_exists: exists };
        Ok(tonic::Response::new(resp))
    }
    async fn edit_user(
        &self,
        request: tonic::Request<pb::User>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let u = request.into_inner();
        let exists = self
            .user_exists(tonic::Request::new(pb::UserExistsRequest {
                email: u.email.clone(),
                nickname: Some(u.nickname.clone()),
                id: Some(u.id.clone()),
            }))
            .await?
            .into_inner();
        if exists.is_exists {
            return Err(tonic::Status::already_exists("昵称已存在"));
        }
        let m = model::User::from(u);
        let rows = db::user::edit(&self.pool, &m)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    async fn delete_or_restore_user(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let pb::DeleteOrRestoreRequest { id, is_del } = request.into_inner();

        let rows = db::user::del_or_restore(&self.pool, id, is_del)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    async fn change_user_status(
        &self,
        request: tonic::Request<pb::ChangeUserStatusRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let pb::ChangeUserStatusRequest { id, status } = request.into_inner();
        let status = model::UserStatus::from(pb::UserStatus::from_i32(status).unwrap());
        let rows = db::user::change_status(&self.pool, id, status)
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    async fn change_user_password(
        &self,
        request: tonic::Request<pb::ChangeUserPasswordRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let request = request.into_inner();
        let rows = db::user::change_password(
            &self.pool,
            request.id,
            request.password,
            request.current_password,
        )
        .await
        .map_err(|e| tonic::Status::internal(e.message))?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    async fn find_user(
        &self,
        request: tonic::Request<pb::FindUserRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindUserResponse>, tonic::Status> {
        let r = request.into_inner();
        let user = db::user::find(&self.pool, &model::UserFindRequest::from(r))
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        let user: Option<pb::User> = match user {
            Some(user) => Some(user.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindUserResponse { user }))
    }
    async fn list_user(
        &self,
        request: tonic::Request<pb::ListUserRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListUserResponse>, tonic::Status> {
        let r = request.into_inner();
        let result = db::user::list(&self.pool, &model::UserListRequest::from(r))
            .await
            .map_err(|e| tonic::Status::internal(e.message))?;
        let paginate = result.to_pb();
        let mut users = Vec::with_capacity(result.data.len());
        for u in result.data {
            let user: pb::User = u.into();
            users.push(user);
        }
        Ok(tonic::Response::new(pb::ListUserResponse {
            paginate: Some(paginate),
            users,
        }))
    }
}
