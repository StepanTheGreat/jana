use std::path;

use crate::utils::silent_command;

pub struct Git {
    git: path::PathBuf,
}

impl Git {
    pub fn get() -> anyhow::Result<Self> {
        let git = which::which("git")?;

        Ok(Self { git })
    }

    /// Initialize a git project at the provided path
    pub fn init(&self, path: &path::Path) {
        let git = self.git.to_str().unwrap();
        let path = path.to_str().unwrap();

        // For now we won't return anything
        let _ = silent_command(git, &["init", path]);
    }
}
