use std::process::Command;

pub fn docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Enable docker tests only when DBNEST_TEST_DOCKER=1 is set
pub fn docker_tests_enabled() -> bool {
    std::env::var("DBNEST_TEST_DOCKER").ok().as_deref() == Some("1") && docker_available()
}
