use actix_web::{post, HttpResponse, Responder};

#[post("/create-database")]
pub async fn create_database(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
