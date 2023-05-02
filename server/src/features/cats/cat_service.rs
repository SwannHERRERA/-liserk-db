use std::sync::Arc;

use crate::{
    core::{Error, QueryParams, ResultPaging},
    models::cat::Cat,
};
use async_trait::async_trait;
use uuid::Uuid;

use super::cat_repo::CatRepo;

#[async_trait]
pub trait CatService: Send + Sync {
    async fn create(&self, cat: &Cat) -> Result<Cat, Error>;

    async fn cats(&self, params: &dyn QueryParams) -> Result<ResultPaging<Cat>, Error>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Cat, Error>;

    async fn update_by_id(&self, id: &str, u: &Cat) -> Result<Cat, Error>;

    async fn delete_by_id(&self, id: &str) -> Result<(), Error>;
}

pub struct UserServiceImpl {
    pub cat_repo: Arc<dyn CatRepo>,
}

#[async_trait]
impl CatService for UserServiceImpl {
    async fn create(&self, cat: &Cat) -> Result<Cat, Error> {
        self.cat_repo.create(&cat).await.map_err(|e| -> Error { e.into() })
    }

    async fn cats(&self, params: &dyn QueryParams) -> Result<ResultPaging<Cat>, Error> {
        self.cat_repo.get_all(params).await.map_err(|e| -> Error { e.into() })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Cat, Error> {
        self.cat_repo.find(id).await.map_err(|e| -> Error { e.into() })
    }

    async fn update_by_id(&self, id: &Uuid, u: &Cat) -> Result<Cat, Error> {
        self.cat_repo.update(id, u).await.map_err(|e| -> Error { e.into() })
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), Error> {
        self.cat_repo.delete(id).await.map_err(|e| -> Error { e.into() })
    }
}
