use std::sync::Arc;

use crate::features::cats::CatService;

pub struct CatContainer {
    pub cat_service: Arc<dyn CatService>,
}
