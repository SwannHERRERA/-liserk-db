use std::fs::remove_dir_all;

use liserk_db::prelude::*;
use liserk_db::infra::{
    generator::{Generator, Randomize},
    postgres::task,
};

#[test]
fn test_create_database() {
    let generator: Generator = Generator::default();
    let folder_name = generator.generate_folder_name();
    let port = generator.generate_port();
    let cluster_creation_result = task::create_cluster(&folder_name);
    println!("{:?}", cluster_creation_result);
    task::start_server(&folder_name, port);
    if let Err(err) = remove_dir_all(f!("{}/folder_name", folder_name)) {
        eprintln!("{:?}", err);
        assert!(false);
    }
}
