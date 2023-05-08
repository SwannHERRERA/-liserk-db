use std::sync::Arc;

use crate::features::users::{DefaultUserService, SqlxUserRepo, UserRepo, UserService};

pub struct CatContainer {
    pub cat_service: Arc<dyn UserService>,
}

impl CatContainer {
    fn new() -> Self {
        let cat_service = Arc::new(DefaultUserService { user_repo: SqlxUserRepo {} });
    }
}

impl Default for CatContainer {
    fn default() -> Self {
        Self::new()
    }
}
