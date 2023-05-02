use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    core::{QueryParams, RepoResult, ResultPaging},
    models::cat::Cat,
};

#[async_trait]
pub trait CatRepo: Send + Sync {
    async fn get_all(&self, params: &dyn QueryParams) -> RepoResult<ResultPaging<Cat>>;

    async fn find(&self, cat_id: &Uuid) -> RepoResult<Cat>;

    async fn find_by_email(&self, email: &str) -> RepoResult<Cat>;

    async fn create(&self, cat: &Cat) -> RepoResult<Cat>;

    async fn update(&self, id: &Uuid, update_user: &Cat) -> RepoResult<Cat>;

    async fn delete(&self, cat_id: &Uuid) -> RepoResult<()>;
}
