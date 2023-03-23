use actix_web::{post, HttpResponse, Responder};

#[post("/create-backup")]
pub async fn create_backup(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
