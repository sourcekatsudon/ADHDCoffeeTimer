use crate::engine::{Checkpoint, Settings};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Saved {
    pub settings: Settings,
    pub checkpoint: Checkpoint,
}

pub fn load(path: &Path) -> Option<Saved> {
    [path.to_path_buf(), path.with_extension("backup")]
        .iter()
        .find_map(|p| {
            let saved: Saved = serde_json::from_slice(&fs::read(p).ok()?).ok()?;
            saved.settings.validate().ok()?;
            Some(saved)
        })
}
pub fn save(path: &PathBuf, saved: &Saved) -> Result<(), String> {
    let write = || -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp = path.with_extension("tmp");
        let mut f = fs::File::create(&temp)?;
        f.write_all(&serde_json::to_vec_pretty(saved)?)?;
        f.sync_all()?;
        // Keep the most recent valid file if a write is interrupted.
        if fs::read(path)
            .ok()
            .and_then(|b| serde_json::from_slice::<Saved>(&b).ok())
            .is_some()
        {
            fs::copy(path, path.with_extension("backup"))?;
        }
        fs::rename(temp, path)?;
        Ok(())
    };
    write().map_err(|_| "errorSave".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atomic_roundtrip_and_backup_recovery() {
        let path =
            std::env::temp_dir().join(format!("ambient-storage-{}.json", std::process::id()));
        let data = Saved {
            settings: Settings::default(),
            checkpoint: Checkpoint {
                remaining: 20.,
                manual_paused: true,
                completed: 3,
            },
        };
        save(&path, &data).unwrap();
        save(&path, &data).unwrap();
        assert!(load(&path).unwrap().checkpoint.manual_paused);
        fs::write(&path, b"broken").unwrap();
        assert_eq!(load(&path).unwrap().checkpoint.remaining, 20.);
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(path.with_extension("backup"));
    }
}
