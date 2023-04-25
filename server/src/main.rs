use std::{process::Command, sync::Arc};

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router, Server,
};
use serde_json::{json, Value};
use shared::{
    generator::{DefaultGenerator, Generator},
    prelude::*,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

type DynGenerator = Arc<dyn Generator + Send + Sync>;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new();
    let db_url = f!(
        "postgres://{user}:{password}@{host}/{db_name}",
        user = "root",
        password = "root",
        host = "localhost:5432",
        db_name = "liserk"
    );
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await;
    let pool = pool.unwrap();

    let generator = Arc::new(DefaultGenerator::default()) as DynGenerator;
    let app = app
        .route("/database/:name", get(create_database))
        .with_state((generator, pool));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn create_database(
    Path(db_name): Path<String>,
    State((generator, _pool)): State<(DynGenerator, PgPool)>,
) -> Json<Value> {
    let metadatas = generate_metadata_for_database(&generator, &db_name);
    // save metadata

    // Warnning: care port use, name used, folder used is case of error for now
    println!("{:?}", metadatas);
    let output = Command::new("docker")
        .arg("run")
        .args(metadatas)
        .output()
        .expect("command should run sucessfuly");
    println!("{:?}", output);
    json!({"a":"b"}).into()
}

fn generate_metadata_for_database(
    generator: &DynGenerator,
    db_name: &String,
) -> [String; 14] {
    const PASSWORD_LENGTH: usize = 12;
    let username = generator.generate_username();
    let password = generator.generate_password(PASSWORD_LENGTH);
    let container_name = generator.generate_container_name();
    let foldername = generator.generate_folder_name();
    let network_port = generator.generate_port();
    let network_port = network_port.to_string();
    let password = f!("POSTGRES_PASSWORD={}", password);
    let username = f!("POSTGRES_USER={}", username);
    let data_path = f!("PGDATA=data/{}", foldername);
    let db = f!("POSTGRES_DB={}", db_name);
    [
        "-p".to_string(),
        network_port,
        "--name".to_string(),
        container_name,
        "-e".to_string(),
        password,
        "-e".to_string(),
        username,
        "-e".to_string(),
        db,
        "-e".to_string(),
        data_path,
        "-d".to_string(),
        "postgres:15".to_string(),
    ]
}
