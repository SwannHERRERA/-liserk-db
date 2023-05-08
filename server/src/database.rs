use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::Postgres;
use sqlx::Pool;

use crate::core::SETTINGS;

lazy_static! {
    pub static ref CONNECTION: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        let db_uri = SETTINGS.database.uri.as_str();
        let db_name = SETTINGS.database.name.as_str();
        Pool::<Postgres>::connect(db_uri)
            .await
            .expect("Failed to initialize MongoDB connection")
    });
}
