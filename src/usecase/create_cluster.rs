use std::thread;

use actix_web::{post, HttpResponse, Responder};
use postgresfixture::{runtime::Runtime, cluster::Cluster};

// Todo: Move postgres custom to postgres task

#[post("/create-cluster")]
pub async fn create_cluster(_req_body: String) -> impl Responder {
    let handle = thread::spawn(|| {
        for runtime in Runtime::find_on_path() {
            println!("{:?}", runtime);
            let cluster = Cluster::new("some/path", runtime);
            cluster.start().unwrap();
            let mut conn = cluster.connect("postgres").unwrap();
            let result = conn.query("SHOW ALL", &[]).unwrap();
            let params: std::collections::HashMap<String, String> = result
                .into_iter()
                .map(|row| (row.get::<usize, String>(0), row.get::<usize, String>(1)))
                .collect();
            println!("{:?}", params);
        }
    });
    // Todo: use the result
    let _ = handle.join();
    HttpResponse::Ok()
}
