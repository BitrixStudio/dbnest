use crate::{DbnestError, Result};
use std::process::Command;

pub fn ensure_docker_available() -> Result<()> {
    let ok = Command::new("docker")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if ok {
        Ok(())
    } else {
        Err(DbnestError::DockerNotAvailable)
    }
}
