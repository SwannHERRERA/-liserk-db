use std::os::unix::fs::PermissionsExt;
use std::{fs::File, io::Write, process::Command};

use crate::{
    infra::generator::{FolderName, NetworkPort, Password, Username},
    prelude::*,
};

pub fn create_cluster(
    data_path: &FolderName,
    username: &Username,
    password: &Password,
) -> Result<String> {
    let output = Command::new("pg_ctl")
        .arg("initdb")
        .arg("-D")
        .arg(f!("data/{}", data_path))
        .arg(f!("-o --encoding=utf8 --locale=C --auth=trust --username={}", username))
        .env("TZ", "UTC")
        .output()?;

    if output.stderr.is_empty() {
        let _log_file = create_log_file(&f!("data/{}/log.txt", data_path));
        create_password_file(data_path, password);

        return Ok(String::from_utf8(output.stdout).expect("Failing to convert to utf8"));
    }
    Err(Error::Generic(
        String::from_utf8(output.stderr).expect("Failing to convert error to utf8"),
    ))
}

fn create_log_file(path: &str) -> Result<File> {
    let file = File::create(path).expect("failling to create the log file");
    let metadata = file.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o666);
    println!("permission: {}", permissions.mode());
    println!("file path: {:?}", file.metadata().unwrap());
    Ok(file)
}

fn create_password_file(data_path: &str, password: &Password) {
    let mut pwfile = File::options()
        .create(true)
        .write(true)
        .open(f!("data/{}/pwfile", data_path))
        .expect("failling to create the password file");

    let _byte_writen = pwfile.write(password.as_bytes()).expect("failed to write password");
}

pub fn start_server(data_path: &str, port: NetworkPort) {
    let output = Command::new("pg_ctl")
        .arg("-D")
        .arg(f!("data/{}", data_path))
        .arg(f!("-o -p {}", port))
        .arg(f!("-l data/{}/log.txt", data_path))
        .arg("start")
        .spawn();
    println!("{:?}", output);
}
