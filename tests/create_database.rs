use std::time::Duration;

use liserk_db::{
    infra::{
        generator::{Generator, Randomize},
        postgres::{cluster::Cluster, runtime::Runtime},
    },
    prelude::*,
};
use sqlx::postgres::PgPoolOptions;

const PASSWORD_LENGTH: usize = 12;

#[sqlx::test]
fn test_create_database() {
    for runtime in Runtime::find_on_path() {
        let generator: Generator = Generator::default();
        let folder_name = generator.generate_folder_name();
        let password = generator.generate_password(PASSWORD_LENGTH);
        let username = generator.generate_username();
        let port = generator.generate_port();
        let data_dir = tempdir::TempDir::new(&folder_name).unwrap();
        let cluster = Cluster::new(data_dir, runtime.clone());
        let _result = cluster.create();
        // cluster.start().unwrap();
        // let databases = cluster.databases();
        // println!("{:?}", databases);
        // let pool = PgPoolOptions::new()
        //     .max_connections(1)
        //     .idle_timeout(Duration::new(1, 0))
        //     .connect(&f!(
        //         "postgres://{}:{}@localhost:{}/postgres",
        //         username,
        //         password,
        //         port
        //     ))
        //     .await
        //     .unwrap();
        //
        // let row: (i64,) = sqlx::query_as("SELECT $1")
        //     .bind(150_i64)
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();
        //
        // assert_eq!(row.0, 150);
    }
}

// #[test]
// fn test_create_database() {
//     let generator: Generator = Generator::default();
//     let folder_name = generator.generate_folder_name();
//     let password = generator.generate_password(PASSWORD_LENGTH);
//     let username = generator.generate_username();
//
//     Cluster::new(folder_name, runtime)
//
//     // let cluster_creation_result =
//     //     task::create_cluster(&folder_name, &username, &password);
//     println!("{:?}", cluster_creation_result);
//     if let Err(err) = remove_dir_all(f!("data/{}", folder_name)) {
//         eprintln!("{:?}", err);
//         assert!(false);
//     }
// }
//
// async fn test_query_database() {
//     let generator: Generator = Generator::default();
//     let folder_name = generator.generate_folder_name();
//     let server_folder_name = folder_name.clone();
//     let username = generator.generate_username();
//     let password = generator.generate_password(PASSWORD_LENGTH);
//     let port = generator.generate_port();
//     let _cluster_creation_result =
//         task::create_cluster(&folder_name, &username, &password);
//     // println!("{:?}", cluster_creation_result);
//     task::start_server(&server_folder_name, port);
//
//     println!("postgres://{}:{}@localhost:{}/postgres", username, password, port);
//
//     let pool = PgPoolOptions::new()
//         .max_connections(1)
//         .idle_timeout(Duration::new(1, 0))
//         .connect(&f!("postgres://{}:{}@localhost:{}/postgres", username, password, port))
//         .await
//         .unwrap();
//
//     let row: (i64,) = sqlx::query_as("SELECT $1")
//         .bind(150_i64)
//         .fetch_one(&pool)
//         .await
//         .unwrap();
//
//     assert_eq!(row.0, 150);
//
//     if let Err(err) = remove_dir_all(f!("data/{}", folder_name)) {
//         eprintln!("{:?}", err);
//         assert!(false);
//     }
//     assert!(false);
// }
