use crate::{DbnestError, Result};
use std::process::{Command, Output};

pub fn run_docker(args: &[&str], hint: &str) -> Result<Output> {
    let out = Command::new("docker").args(args).output()?;

    if out.status.success() {
        return Ok(out);
    }

    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
    let command = format!("docker {}", args.join(" "));

    Err(DbnestError::DockerCommandFailed {
        command,
        stderr,
        hint: hint.to_string(),
    })
}
