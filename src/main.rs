use actix_web::{App, HttpServer};
use usecase::create_cluster::create_cluster;
use usecase::create_instance::create_instance;
use usecase::heartbeat::heartbeat;
use std::io;

mod prelude;
mod infra;
mod usecase;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(heartbeat)
            .service(create_cluster)
            .service(create_instance)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
