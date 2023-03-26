use crate::infra::{postgres::task, generator::{Generator, Randomize}};
use actix_web::{post, HttpResponse, Responder};
use log::info;

#[post("/create-cluster")]
pub async fn create_cluster(_req_body: String) -> impl Responder {
    let generator = Generator::default();
    let folder_name = generator.generate_folder_name();
    let r = task::create_cluster(&folder_name);
    info!("{:?}", r);
    HttpResponse::Ok()
}
