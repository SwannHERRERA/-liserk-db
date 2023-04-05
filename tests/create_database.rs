use std::fs::remove_dir_all;

use liserk_db::infra::{
    generator::{Generator, Randomize},
    postgres::task,
};
use liserk_db::prelude::*;
use sqlx::postgres::PgPoolOptions;

const PASSWORD_LENGTH: usize = 12;

// #[test]
// fn test_create_database() {
//     let generator: Generator = Generator::default();
//     let folder_name = generator.generate_folder_name();
//     let password = generator.generate_password(PASSWORD_LENGTH);
//     let username = generator.generate_username();
//     let port = generator.generate_port();
//     let cluster_creation_result = task::create_cluster(&folder_name, &username, &password);
//     println!("{:?}", cluster_creation_result);
//     task::start_server(&folder_name, port);
//     if let Err(err) = remove_dir_all(f!("data/{}", folder_name)) {
//         eprintln!("{:?}", err);
//         assert!(false);
//     }
// }
//
//
// #[sqlx::test]
// async fn test_query_database() {
//     let generator: Generator = Generator::default();
//     let folder_name = generator.generate_folder_name();
//     let username = generator.generate_username();
//     let password = generator.generate_password(PASSWORD_LENGTH);
//     let port = generator.generate_port();
//     let cluster_creation_result = task::create_cluster(&folder_name, &username, &password);
//     println!("{:?}", cluster_creation_result);
//     task::start_server(&folder_name, port);
//
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&f!("postgres://{}:{}@localhost:{}/test", username, password, port)).await.unwrap();
//
//     let row: (i64,) = sqlx::query_as("SELECT $1")
//         .bind(150_i64)
//         .fetch_one(&pool).await.unwrap();
//
//     assert_eq!(row.0, 150);
//
//     if let Err(err) = remove_dir_all(f!("data/{}", folder_name)) {
//         eprintln!("{:?}", err);
//         assert!(false);
//     }
// }
