use actix_web::{post, HttpResponse, Responder};

#[post("/create-cluster")]
pub async fn create_cluster(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
