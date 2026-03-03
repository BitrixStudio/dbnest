use crate::{Result, error::DbnestError};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Dirs {
    pub base: PathBuf,
    pub instances: PathBuf,
    pub sqlite: PathBuf,
}

impl Dirs {
    pub fn from_base(base: PathBuf) -> Self {
        let instances = base.join("instances");
        let sqlite = base.join("sqlite");
        Self {
            base,
            instances,
            sqlite,
        }
    }
    pub fn for_app() -> Result<Self> {
        let proj = ProjectDirs::from("dev", "dbnest", "dbnest").ok_or_else(|| {
            DbnestError::InvalidArgument("cannot determine OS data directory".into())
        })?;

        let base = proj.data_dir().to_path_buf();
        let instances = base.join("instances");
        let sqlite = base.join("sqlite");

        Ok(Self {
            base,
            instances,
            sqlite,
        })
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.instances)?;
        std::fs::create_dir_all(&self.sqlite)?;
        Ok(())
    }

    pub fn instance_file(&self, id: &str) -> PathBuf {
        self.instances.join(format!("{id}.json"))
    }

    pub fn managed_sqlite_dir(&self, id: &str) -> PathBuf {
        self.sqlite.join(id)
    }

    pub fn managed_sqlite_file(&self, id: &str) -> PathBuf {
        self.managed_sqlite_dir(id).join("database.sqlite")
    }

    pub fn is_within_base(&self, p: &Path) -> bool {
        p.starts_with(&self.base)
    }
}
