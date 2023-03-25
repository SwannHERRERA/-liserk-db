use actix_web::{head, HttpResponse, Responder};

#[head("/heartbeat")]
pub async fn heartbeat(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
