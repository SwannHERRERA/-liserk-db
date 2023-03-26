use actix_web::{post, HttpResponse, Responder, web};
use log::info;

use crate::infra::{postgres::task, generator::{Generator, Randomize}};

#[post("/start-server")]
pub async fn start_server(path: web::Path<String>, _req_body: String) -> impl Responder {
    let generator = Generator::default(); 
    task::start_server(&path.into_inner(), generator.generate_port());
    info!("task server start");
    HttpResponse::Ok()
}
