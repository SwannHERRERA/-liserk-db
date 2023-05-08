use std::sync::Arc;

use crate::{
    core::{Error, QueryParams, ResultPaging},
    models::user::User,
};
use async_trait::async_trait;
use uuid::Uuid;

use super::user_repo::UserRepo;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn save(&self, cat: &User) -> Result<User, Error>;

    async fn get_all(
        &self,
        params: &dyn QueryParams,
    ) -> Result<ResultPaging<User>, Error>;

    async fn find_by_id(&self, id: &Uuid) -> Result<User, Error>;

    async fn update_by_id(&self, id: &Uuid, u: &User) -> Result<User, Error>;

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), Error>;
}

pub struct DefaultUserService {
    pub user_repo: Arc<dyn UserRepo>,
}

#[async_trait]
impl UserService for DefaultUserService {
    async fn save(&self, cat: &User) -> Result<User, Error> {
        self.user_repo.create(&cat).await.map_err(|e| -> Error { e.into() })
    }

    async fn get_all(
        &self,
        params: &dyn QueryParams,
    ) -> Result<ResultPaging<User>, Error> {
        self.user_repo
            .get_all(params)
            .await
            .map_err(|e| -> Error { e.into() })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<User, Error> {
        self.user_repo.find(id).await.map_err(|e| -> Error { e.into() })
    }

    async fn update_by_id(&self, id: &Uuid, u: &User) -> Result<User, Error> {
        self.user_repo.update(id, u).await.map_err(|e| -> Error { e.into() })
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), Error> {
        self.user_repo.delete(id).await.map_err(|e| -> Error { e.into() })
    }
}
