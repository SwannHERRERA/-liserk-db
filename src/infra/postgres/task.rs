use std::process::Command;

use crate::prelude::*;

pub fn create_cluster(data_path: &str) -> Result<bool> {
    Command::new("pg_ctl").arg("-D").arg(data_path).arg("initdb").output()?;
    Ok(true)
}
