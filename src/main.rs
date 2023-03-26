use actix_web::{App, HttpServer};
use std::io;
use std::time::SystemTime;
use usecase::create_cluster::create_cluster;
use usecase::start_server::start_server;
use usecase::heartbeat::heartbeat;

mod infra;
mod prelude;
mod usecase;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let Ok(_) = setup_logger() else {
        panic!("failed to setup the logger");
    };
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
