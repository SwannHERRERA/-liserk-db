use actix_web::{post, HttpResponse, Responder};

use crate::infra::postgres::task;

#[post("/start-server")]
pub async fn start_server(_req_body: String) -> impl Responder {
    task::start_server("data", 5432);
    HttpResponse::Ok()
}
