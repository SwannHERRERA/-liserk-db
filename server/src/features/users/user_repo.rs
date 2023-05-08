use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    core::{QueryParams, RepoResult, ResultPaging},
    database::CONNECTION,
    models::user::User,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEntity {
    email: String,
    name: String,
    id_user: Uuid,
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_all(
        &self,
        params: &dyn QueryParams,
    ) -> RepoResult<ResultPaging<UserEntity>>;

    async fn find(&self, user_id: &Uuid) -> RepoResult<UserEntity>;

    async fn find_by_email(&self, email: &str) -> RepoResult<UserEntity>;

    async fn create(&self, user: &User) -> RepoResult<UserEntity>;

    async fn update(&self, id: &Uuid, update_user: &User) -> RepoResult<UserEntity>;

    async fn delete(&self, user_id: &Uuid) -> RepoResult<()>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SqlxUserRepo;

#[async_trait]
impl UserRepo for SqlxUserRepo {
    async fn get_all(
        &self,
        params: &dyn QueryParams,
    ) -> RepoResult<ResultPaging<UserEntity>> {
        let pool = CONNECTION.get().await;
        sqlx::query_as!(UserEntity, "SELECT id_user, email, name FROM USERS")
            .fetch_all(pool)
            .await
            .map_err(|err| err.into())
    }

    async fn find(&self, cat_id: &Uuid) -> RepoResult<User> {
        let pool = CONNECTION.get().await;
        let result =
            sqlx::query_as!(UserEntity, "SELECT id_user, email, name FROM USERS")
                .fetch_one(pool)
                .await;
    }

    async fn find_by_email(&self, email: &str) -> RepoResult<UserEntity> {
        todo!()
    }

    async fn create(&self, user: &User) -> RepoResult<UserEntity> {
        todo!()
    }

    async fn update(&self, id: &Uuid, update_user: &User) -> RepoResult<UserEntity> {
        todo!()
    }

    async fn delete(&self, user_id: &Uuid) -> RepoResult<()> {
        todo!()
    }
}
