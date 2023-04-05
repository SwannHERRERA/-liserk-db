use crate::infra::{
    generator::{Generator, Randomize},
    postgres::task,
};
use actix_web::{post, HttpResponse, Responder};
use log::info;

const CLUSTER_DB_USER_PASSWORD_LENGTH: usize = 24;

#[post("/create-cluster")]
pub async fn create_cluster(_req_body: String) -> impl Responder {
    let generator = Generator::default();
    let folder_name = generator.generate_folder_name();
    let username = generator.generate_username();
    let password = generator.generate_password(CLUSTER_DB_USER_PASSWORD_LENGTH);
    let cluster = task::create_cluster(&folder_name, &username, &password);
    info!("{:?}", cluster);
    HttpResponse::Ok()
}
