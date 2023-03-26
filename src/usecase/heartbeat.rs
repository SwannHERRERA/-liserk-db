use actix_web::{head, HttpResponse, Responder};

#[head("/heartbeat")]
pub async fn heartbeat() -> impl Responder {
    HttpResponse::NoContent()
}
