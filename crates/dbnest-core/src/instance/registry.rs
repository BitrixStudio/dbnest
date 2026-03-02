use crate::{paths::Dirs, Result};
use fs2::FileExt;
use std::{fs::File, io::Write};

use super::model::Instance;

#[derive(Debug, Clone)]
pub struct Registry {
    dirs: Dirs,
}

impl Registry {
    pub fn new() -> Result<Self> {
        let dirs = Dirs::for_app()?;
        dirs.ensure()?;
        Ok(Self { dirs })
    }

    pub fn dirs(&self) -> &Dirs {
        &self.dirs
    }

    pub fn write(&self, inst: &Instance) -> Result<()> {
        let path = self.dirs.instance_file(&inst.id);
        let lock_path = self.dirs.base.join(".registry.lock");

        let lock = File::create(lock_path)?;
        
        // prevent concurrent writes
        lock.lock_exclusive()?;

        let json = serde_json::to_vec_pretty(inst)?;
        let mut f = File::create(path)?;
        f.write_all(&json)?;
        f.write_all(b"\n")?;

        lock.unlock()?;
        Ok(())
    }

    pub fn read(&self, id: &str) -> Result<Instance> {
        let path = self.dirs.instance_file(id);
        let bytes = std::fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub fn remove_metadata(&self, id: &str) -> Result<()> {
        let path = self.dirs.instance_file(id);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Instance>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(&self.dirs.instances)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let bytes = std::fs::read(&p)?;
            match serde_json::from_slice::<Instance>(&bytes) {
                Ok(i) => out.push(i),
                Err(_) => {
                    // Ignore corrupted entries for now (can add warnings later)
                    continue;
                }
            }
        }
        out.sort_by_key(|i| std::cmp::Reverse(i.created_at));
        Ok(out)
    }
}