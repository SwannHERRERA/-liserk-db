use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router, Server,
};
use routes::database::{create_database, list_database};
use shared::{
    generator::{DefaultGenerator, Generator},
    prelude::*,
};
use sqlx::PgPool;

mod errors;
mod logger;
mod routes;
mod settings;

type DynGenerator = Arc<dyn Generator + Send + Sync>;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new();
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url =
        std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let pool = PgPool::connect(&db_url).await;
    let pool = pool.expect("failed to connect with database");

    let generator = Arc::new(DefaultGenerator::default()) as DynGenerator;
    let app = app
        .route("/database/:name", post(create_database))
        .route("/database", get(list_database))
        .with_state((generator, pool));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
