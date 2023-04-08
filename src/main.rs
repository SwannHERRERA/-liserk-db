use actix_web::{App, HttpServer};
use liserk_db::infra::generator::{Generator, Randomize};
use liserk_db::infra::postgres::task;
use sqlx::postgres::PgPoolOptions;
use std::fs::remove_dir_all;
use std::io;
use std::time::{SystemTime, Duration};
use usecase::create_cluster::create_cluster;
use usecase::heartbeat::heartbeat;
use usecase::start_server::start_server;
use prelude::f;

mod infra;
mod prelude;
mod usecase;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    let generator: Generator = Generator::default();
    let folder_name = generator.generate_folder_name();
    let username = generator.generate_username();
    let password = generator.generate_password(12);
    let port = generator.generate_port();
    let cluster_creation_result = task::create_cluster(&folder_name, &username, &password);
    println!("{:?}", cluster_creation_result);
    task::start_server(&folder_name, port);

    println!("postgres://{}:{}@localhost:{}/postgres", username, password, port);

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(Duration::new(1, 0))
        .connect(&f!("postgres://{}:{}@localhost:{}/postgres", username, password, port)).await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await.unwrap();

    assert_eq!(row.0, 150);

    if let Err(err) = remove_dir_all(f!("data/{}", folder_name)) {
        eprintln!("{:?}", err);
        assert!(false);
    }
}

// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     let Ok(_) = setup_logger() else {
//         panic!("failed to setup the logger");
//     };
//     HttpServer::new(|| {
//         App::new()
//             .service(heartbeat)
//             .service(create_cluster)
//             .service(start_server)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
