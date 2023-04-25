use axum::{routing::get, Json, Router, Server};
use serde_json::{json, Value};
use shared::prelude::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new();
    let app = app.route("/", get(create_database));
    let db_url = f!(
        "postgres://{user}:{password}@{host}/{db_name}",
        user = "root",
        password = "root",
        host = "localhost:5432",
        db_name = "liserk"
    );
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await;
    let pool = pool.unwrap();

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn create_database() -> Json<Value> {
    // use terra to build the docker compose and run it
    // maybe i should use a command
    // I should use a database for storing element.
    json!({"a":"b"}).into()
}
