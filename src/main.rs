use actix_web::{App, HttpServer};
use std::io;
use usecase::create_cluster::create_cluster;
use usecase::start_server::start_server;
use usecase::heartbeat::heartbeat;

mod infra;
mod prelude;
mod usecase;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(heartbeat)
            .service(create_cluster)
            .service(start_server)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
