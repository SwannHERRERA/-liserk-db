use std::{fs::File, process::Command, io::Write};

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

    // println!("output: {:?}", output);

    if output.stderr.is_empty() {
        // println!("data/{}/log.txt", data_path);
        File::create(f!("data/{}/log.txt", data_path))
            .expect("failling to create the log file");
        let mut pwfile = File::options()
            .create(true)
            .write(true)
            .open(f!("data/{}/pwfile", data_path)).expect("failling to create the password file");

        pwfile.write(password.as_bytes()).expect("failed to write password");

        return Ok(String::from_utf8(output.stdout).expect("Failing to convert to utf8"));
    }
    Err(Error::Generic(
        String::from_utf8(output.stderr).expect("Failing to convert error to utf8"),
    ))
}

pub fn start_server(data_path: &str, port: NetworkPort) {
    let mut command = Command::new("pg_ctl");
        command.arg("-D")
        .arg(f!("data/{}", data_path))
        .arg(f!("-o \"-p {port}\""))
        .arg(f!("-l data/{}/log.txt", data_path))
        .arg("start");
        
    eprintln!("{:?}", command);
        
    let _output = Command::new("pg_ctl")
        .arg("-D")
        .arg(f!("data/{}", data_path))
        .arg(f!("-o \"-p {}\"", port))
        .arg(f!("-l data/{}/log.txt", data_path))
        .arg("start")
        .output();
    // println!("{:?}", output);
}
