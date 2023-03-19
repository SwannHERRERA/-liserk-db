use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use usecase::create_database::create_database;
use std::io;
mod prelude;
mod usecase;


#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(create_database)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
