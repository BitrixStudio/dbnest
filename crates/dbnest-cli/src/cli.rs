use clap::{Parser, Subcommand};
use dbnest_core::{Engine, Instance, InstanceSpec, SqliteSpec};

#[derive(Debug, Parser)]
#[command(name = "dbnest", version, about = "Cozy local databases in seconds")]
pub struct Root {
    #[arg(long)]
    pub json: bool,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Up(UpArgs),
    Ls(LsArgs),
    Stop(StopArgs),
    Rm(RmArgs),
}

#[derive(Debug, Parser)]
pub struct UpArgs {
    pub engine: String,

    /// SQLite file path (only for sqlite)
    /// If omitted, dbnest manages a file under app data dir
    #[arg(long)]
    pub path: Option<std::path::PathBuf>,
}

impl UpArgs {
    pub fn run(self) -> dbnest_core::Result<Instance> {
        let engine = parse_engine(&self.engine)?;
        let spec = match engine {
            Engine::Sqlite => InstanceSpec {
                engine,
                sqlite: Some(SqliteSpec { path: self.path }),
            },
            Engine::Postgres | Engine::Mysql => InstanceSpec { engine, sqlite: None },
        };
        dbnest_core::provision(spec)
    }
}

#[derive(Debug, Parser)]
pub struct LsArgs {}

impl LsArgs {
    pub fn run(self) -> dbnest_core::Result<Vec<dbnest_core::InstanceSummary>> {
        dbnest_core::list_instances()
    }
}

#[derive(Debug, Parser)]
pub struct StopArgs {
    pub id: String,
}
impl StopArgs {
    pub fn run(self) -> dbnest_core::Result<()> {
        dbnest_core::stop_instance(&self.id)
    }
}

#[derive(Debug, Parser)]
pub struct RmArgs {
    pub id: String,
}
impl RmArgs {
    pub fn run(self) -> dbnest_core::Result<()> {
        dbnest_core::remove_instance(&self.id)
    }
}

fn parse_engine(s: &str) -> dbnest_core::Result<Engine> {
    match s.to_lowercase().as_str() {
        "sqlite" => Ok(Engine::Sqlite),
        "postgres" | "pg" => Ok(Engine::Postgres),
        "mysql" => Ok(Engine::Mysql),
        _ => Err(dbnest_core::DbnestError::InvalidArgument(format!(
            "Unknown engine '{s}'. Expected: sqlite|postgres|mysql"
        ))),
    }
}