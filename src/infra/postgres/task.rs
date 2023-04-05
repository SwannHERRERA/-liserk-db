use std::{fs::File, process::Command};

use crate::{
    infra::generator::{FolderName, NetworkPort, Password, Username},
    prelude::*,
};

pub fn create_cluster(
    data_path: &FolderName,
    username: &Username,
    _password: &Password,
) -> Result<String> {
    let output = Command::new("pg_ctl")
        .arg("initdb")
        .arg("-D")
        .arg(f!("data/{}", data_path))
        .arg("-o --encoding=utf8 --locale=C --auth=trust")
        .arg("--username")
        .arg(username)
        .env("TZ", "UTC")
        .output()?;

    println!("output: {:?}", output);

    if output.stderr.is_empty() {
        println!("data/{}/log.txt", data_path);
        File::create(f!("data/{}/log.txt", data_path))
            .expect("failling to create the log file");
        return Ok(String::from_utf8(output.stdout).expect("Failing to convert to utf8"));
    }
    Err(Error::Generic(
        String::from_utf8(output.stderr).expect("Failing to convert error to utf8"),
    ))
}

pub fn start_server(data_path: &str, port: NetworkPort) {
    let output = Command::new("pg_ctl")
        .arg("-D")
        .arg(f!("data/{}", data_path))
        .arg(f!("-l data/{}/log.txt", data_path))
        .arg(f!("-o \"-p {port}\""))
        .arg("start")
        .output();
    println!("{:?}", output);
}
