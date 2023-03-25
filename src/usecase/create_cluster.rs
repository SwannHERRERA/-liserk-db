use actix_web::{post, HttpResponse, Responder};
use crate::infra::postgres::task;

#[post("/create-cluster")]
pub async fn create_cluster(_req_body: String) -> impl Responder {
    let r = task::create_cluster("data/");
    println!("{:?}, task created successfully", r);
    HttpResponse::Ok()
}
