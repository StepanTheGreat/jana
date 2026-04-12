use std::{env, io, path, process};

/// A set of java programs used for building java files
pub struct Java {
    java: path::PathBuf,
    javac: path::PathBuf,
    jar: path::PathBuf,
}

impl Java {
    /// Get a java instance if it's present on the machine
    pub fn get() -> anyhow::Result<Self> {
        // If JAVA_HOME is invalid, this tool will think that java doesn't actually exist
        let java_home = env::var("JAVA_HOME").map(|path| path::PathBuf::from(path).join("./bin"));

        let java_path = java_home.clone().unwrap_or_default().join("java");
        let javac_path = java_home.clone().unwrap_or_default().join("javac");
        let jar_path = java_home.unwrap_or_default().join("jar");

        let java = which::which(java_path)?;
        let javac = which::which(javac_path)?;
        let jar = which::which(jar_path)?;

        Ok(Self { java, javac, jar })
    }

    /// Compile provided source file **pattern** into the provided output directory
    pub fn compile(&self, sources_file: &path::Path, dst: &path::Path) -> io::Result<()> {
        process::Command::new(&self.javac)
            .arg("-d")
            .arg(dst)
            .arg(format!("@{}", sources_file.to_string_lossy()))
            .status()
            .map(|_| ())
    }

    /// Run provided class within provided class-path directory
    pub fn run(&self, class_path: &path::Path, class: &str) -> io::Result<()> {
        process::Command::new(&self.java)
            .arg("-cp")
            .arg(class_path)
            .arg(class)
            .status()
            .map(|_| ())
    }
}
