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
    Plan(PlanArgs),
    Apply(ApplyArgs),
    Status(StatusArgs),
}

#[derive(Debug, Parser)]
pub struct UpArgs {
    pub engine: String,

    /// SQLite file path (only for sqlite)
    /// If omitted, dbnest manages a file under app data dir
    #[arg(long)]
    pub path: Option<std::path::PathBuf>,
    #[arg(long)]
    pub user: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
    #[arg(long)]
    pub db: Option<String>,
    #[arg(long)]
    pub image: Option<String>,
    #[arg(long)]
    pub schema: Option<std::path::PathBuf>,
}

impl UpArgs {
    pub fn run(self) -> dbnest_core::Result<Instance> {
        let engine = parse_engine(&self.engine)?;
        let spec = match engine {
            Engine::Sqlite => InstanceSpec {
                engine,
                sqlite: Some(SqliteSpec { path: self.path }),
                postgres: None,
            },
            Engine::Postgres => {
                let user = self.user.ok_or_else(|| {
                    dbnest_core::DbnestError::InvalidArgument(
                        "--user is required for postgres".into(),
                    )
                })?;
                let password = self.password.ok_or_else(|| {
                    dbnest_core::DbnestError::InvalidArgument(
                        "--password is required for postgres".into(),
                    )
                })?;
                let db = self.db.ok_or_else(|| {
                    dbnest_core::DbnestError::InvalidArgument(
                        "--db is required for postgres".into(),
                    )
                })?;

                InstanceSpec {
                    engine,
                    sqlite: None,
                    postgres: Some(dbnest_core::instance::PostgresSpec {
                        user,
                        password,
                        db,
                        image: self.image,
                    }),
                }
            }
            Engine::Mysql => InstanceSpec {
                engine,
                sqlite: None,
                postgres: None,
            },
        };
        dbnest_core::provision_with_schema(spec, self.schema.as_deref())
    }
}

#[derive(Debug, clap::Parser)]
pub struct StatusArgs {
    /// Instance id (omit when using --all)
    pub id: Option<String>,

    /// Show status for all instances
    #[arg(long)]
    pub all: bool,
}

impl StatusArgs {
    pub fn run(self) -> dbnest_core::Result<StatusResult> {
        if self.all {
            let reports = dbnest_core::status_all()?;
            Ok(StatusResult::Many(reports))
        } else {
            let id = self.id.ok_or_else(|| {
                dbnest_core::DbnestError::InvalidArgument("provide <id> or use --all".into())
            })?;
            let report = dbnest_core::status_one(&id)?;
            Ok(StatusResult::One(report))
        }
    }
}

pub enum StatusResult {
    One(dbnest_core::InstanceStatusReport),
    Many(Vec<dbnest_core::InstanceStatusReport>),
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

#[derive(Debug, Parser)]
pub struct PlanArgs {
    pub engine: String,
    #[arg(long)]
    pub schema: std::path::PathBuf,
}
impl PlanArgs {
    pub fn run(self) -> dbnest_core::Result<dbnest_core::schema::plan::SqlPlan> {
        let engine = parse_engine(&self.engine)?;
        dbnest_core::plan_schema(engine, &self.schema)
    }
}

#[derive(Debug, Parser)]
pub struct ApplyArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub schema: std::path::PathBuf,
}
impl ApplyArgs {
    pub fn run(self) -> dbnest_core::Result<()> {
        dbnest_core::apply_schema_to_instance(&self.id, &self.schema)
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
