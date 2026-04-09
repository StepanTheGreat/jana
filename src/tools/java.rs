use std::{env, path::PathBuf};

/// A set of java programs used for building java files
pub struct Java {
    javac: PathBuf,
    jar: PathBuf
}

impl Java {
    /// Get a java instance if it's present on the machine
    pub fn get() -> anyhow::Result<Self> {

        // If JAVA_HOME is invalid, this tool will think that java doesn't actually exist
        let java_home = env::var("JAVA_HOME")
            .map(|path| PathBuf::from(path).join("./bin"));

        let javac_path = java_home.clone().unwrap_or_default().join("javac");
        let jar_path = java_home.unwrap_or_default().join("jar");

        let javac = which::which(javac_path)?;
        let jar = which::which(jar_path)?;

        Ok(Self {
            javac,
            jar
        })
    }
}