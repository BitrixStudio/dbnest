use crate::{
    Backend, ConnectionInfo, ContainerInfo,
    engine::Engine,
    error::{DbnestError, Result},
    ids::new_instance_id,
    instance::Registry,
    instance::{Instance, InstanceSpec},
    runtime::docker::ensure_docker_available,
};
use std::net::TcpListener;
use std::process::Command;
use time::OffsetDateTime;

pub fn provision_postgres_docker(spec: InstanceSpec) -> Result<Instance> {
    if spec.engine != Engine::Postgres {
        return Err(DbnestError::InvalidArgument(
            "provision_postgres_docker called for non-postgres".into(),
        ));
    }

    ensure_docker_available()?;

    let pg = spec.postgres.ok_or_else(|| {
        DbnestError::InvalidArgument("postgres spec missing (user/password/db)".into())
    })?;

    let id = new_instance_id();
    let registry = Registry::new()?;

    let image = pg.image.unwrap_or_else(|| "postgres:16-alpine".to_string());

    let host_port = pick_free_port()?;
    let container_name = format!("dbnest-postgres-{id}");

    // Run postgres container
    // Expose only to localhost
    let output = Command::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            &container_name,
            "--label",
            "dbnest.managed=true",
            "--label",
            &format!("dbnest.id={id}"),
            "--label",
            "dbnest.engine=postgres",
            "-e",
            &format!("POSTGRES_USER={}", pg.user),
            "-e",
            &format!("POSTGRES_PASSWORD={}", pg.password),
            "-e",
            &format!("POSTGRES_DB={}", pg.db),
            "-p",
            &format!("127.0.0.1:{host_port}:5432"),
            image.as_str(),
        ])
        .output()?;

    let cmd_pretty =
        format!("docker run -d --name {container_name} -p 127.0.0.1:{host_port}:5432 {image}");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DbnestError::DockerCommandFailed {
            command: cmd_pretty,
            stderr,
            hint: "Check Docker is running (try `docker ps`). On Windows/macOS ensure Docker Desktop is started.".into(),
        });
    }

    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Wait until ready
    wait_for_postgres_ready(host_port, &pg.user, &pg.password, &pg.db)?;

    let database_url = format!(
        "postgres://{}:{}@127.0.0.1:{}/{}",
        urlencoding::encode(&pg.user),
        urlencoding::encode(&pg.password),
        host_port,
        pg.db
    );

    let inst = Instance {
        id: id.clone(),
        engine: Engine::Postgres,
        backend: Backend::Container,
        created_at: OffsetDateTime::now_utc(),
        connection: ConnectionInfo {
            database_url,
            host: Some("127.0.0.1".into()),
            port: Some(host_port),
            database: Some(pg.db),
            user: Some(pg.user),
        },
        sqlite: None,
        container: Some(ContainerInfo {
            runtime: "docker".into(),
            container_id,
            image,
        }),
    };

    registry.write(&inst)?;
    Ok(inst)
}

fn pick_free_port() -> Result<u16> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    Ok(listener.local_addr()?.port())
}

fn wait_for_postgres_ready(port: u16, user: &str, password: &str, db: &str) -> Result<()> {
    let url = format!(
        "postgres://{}:{}@127.0.0.1:{}/{}",
        urlencoding::encode(user),
        urlencoding::encode(password),
        port,
        db
    );

    for _ in 0..60 {
        if can_connect(&url) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    Err(DbnestError::InvalidArgument(
        "postgres did not become ready in time".into(),
    ))
}

fn can_connect(url: &str) -> bool {
    match postgres::Client::connect(url, postgres::NoTls) {
        Ok(mut c) => c.simple_query("SELECT 1;").is_ok(),
        Err(_) => false,
    }
}
