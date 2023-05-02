use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sea_orm::{Database, DatabaseConnection};

use crate::settings::SETTINGS;

lazy_static! {
    pub static ref CONNECTION: AsyncOnce<DatabaseConnection> = AsyncOnce::new(async {
        let db_uri = SETTINGS.database.uri.as_str();
        let db_name = SETTINGS.database.name.as_str();

        Database::connect(db_uri)
            .await
            .expect("Failed to initialize MongoDB connection")
    });
}
