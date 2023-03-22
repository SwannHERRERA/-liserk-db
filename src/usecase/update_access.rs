use actix_web::{post, HttpResponse, Responder};

#[post("/create-instance")]
pub async fn create_instance(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

