use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::date;
use crate::utils::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Cat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub user: Uuid,
    pub name: String,
    pub updated_at: Date,
    pub created_at: Date,
}

impl Cat {
    pub fn new(user: Uuid, name: String) -> Self {
        let now = date::now();
        Self {
            id: None,
            user,
            name,
            updated_at: now,
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicCat {
    pub id: Uuid,
    pub user: Uuid,
    pub name: String,
    pub updated_at: Date,
    pub created_at: Date,
}

impl From<Cat> for PublicCat {
    fn from(cat: Cat) -> Self {
        Self {
            id: cat.id.unwrap(),
            user: cat.user,
            name: cat.name.clone(),
            updated_at: cat.updated_at,
            created_at: cat.created_at,
        }
    }
}
