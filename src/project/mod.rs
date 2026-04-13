use std::{
    collections::HashMap,
    fs, io,
    path::{self, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    consts::{PROJECT_FILE, PROJECT_SRC, PROJECT_TARGET, PROJECT_TMP, TEMP_FILE},
    utils::{awrite_file, get_or_make_dir},
};

/// Supported project languages
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectLanguage {
    #[serde(rename = "java")]
    Java,

    #[serde(rename = "kotlin")]
    Kotlin,
}

impl ProjectLanguage {
    /// Get language's name as string
    fn as_str(&self) -> &str {
        match self {
            Self::Java => "java",
            Self::Kotlin => "kotlin",
        }
    }

    /// Get language's extensions string
    pub fn ext_str(&self) -> &str {
        match self {
            Self::Java => "java",
            Self::Kotlin => "kt",
        }
    }
}

/// The [lib] section of the package
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectLibSection {
    pub path: PathBuf
}

/// The [[bin]] section of the package
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBinSection {
    pub name: String,
    pub path: PathBuf
}

/// The [package] section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPackageSection {
    pub package: String,
    pub version: String,
    pub language: ProjectLanguage,
}

/// A single dependency value
#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct ProjectDependency(pub String);

/// A list of project dependencies
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[repr(transparent)]
pub struct ProjectDependenciesSection(pub HashMap<String, ProjectDependency>);

/// The root files which groups all sub entries
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectFile {
    pub package: ProjectPackageSection,
    pub lib: Option<ProjectLibSection>,
    pub bin: Vec<ProjectBinSection>,
    pub dependencies: Option<ProjectDependenciesSection>,
    pub dev_dependencies: Option<ProjectDependenciesSection>,
}

/// Read the project file from the provided **project path**
pub fn read_project_file(path: &path::Path) -> anyhow::Result<ProjectFile> {
    let src = fs::read(path.join(PROJECT_FILE))?;
    let file = toml::from_slice::<ProjectFile>(&src)?;

    Ok(file)
}

/// Write a project file at the provided **project path**
pub fn write_project_file(path: &path::Path, file: &ProjectFile) -> anyhow::Result<()> {
    let src = toml::to_string_pretty(file)?;

    // Atomically write to our project file using our temporary folder
    awrite_file(path.join(PROJECT_FILE), src, get_project_tmp_file(path)?)?;

    Ok(())
}

/// Check whether the provided directory contains a jana project
///
/// A jana project in general is a directory with a Jana.toml file. The premise of the check is to simply see if a project file exists,
/// regardless if it might not even work.
///
/// This might return an error in case the directory can't even be accessed.
pub fn is_project_dir(path: &path::Path) -> io::Result<bool> {
    let conf_path = path.join(PROJECT_FILE);

    fs::exists(conf_path)
}

/// Get project's target directory path.
///
/// If it doesn't exist - create one, in any other case just use an existing one
pub fn get_project_target(path: &path::Path) -> io::Result<path::PathBuf> {
    get_or_make_dir(&path.join(PROJECT_TARGET))
}

/// Get project's temp folder's path at the provided project path
///
/// Can fail if the path is unreachable
fn get_project_tmp(path: &path::Path) -> io::Result<path::PathBuf> {
    get_or_make_dir(&path.join(PROJECT_TARGET).join(PROJECT_TMP))
}

/// Get project's common temp file path (the most common temp file to be used for all operations)
///
/// Can fail if the project path is unreachable
pub fn get_project_tmp_file(path: &path::Path) -> io::Result<path::PathBuf> {
    get_project_tmp(path).map(|p| p.join(TEMP_FILE))
}

/// Get a list of all source files with the provided extension from the project at the specified path. This will
/// return a list of absolute source file paths.
///
/// Note that this can fail for multiple reasons:
/// 1. The project path doesn't have a `src` directory
/// 2. The `src` file is not a directory file
/// 3. Files within have invalid unicode
/// 4. File permissions
///
/// Usually it's better to propagate these errors to the user to let them sort everything out
pub fn get_project_source_files(
    path: &path::Path,
    ext: &str,
) -> anyhow::Result<Vec<path::PathBuf>> {
    // Our output files
    let mut src_files = Vec::with_capacity(16);

    // We're going to traverse files depth-first
    let mut dir_stack: Vec<PathBuf> = Vec::with_capacity(16);
    dir_stack.push(path.join(PROJECT_SRC));

    while let Some(current_dir) = dir_stack.pop() {
        // For each of its files
        for fentry in fs::read_dir(current_dir)? {
            let file = fentry?;

            let fpath = file.path();
            let ftype = file.file_type()?;

            if ftype.is_dir() {
                // If it's a directory - push it into the stack
                dir_stack.push(fpath);
            } else if ftype.is_file() {
                // In other case check its extension and if it matches - push into the source files list

                let fname = file.file_name();
                let fname = fname.to_str().ok_or(anyhow::anyhow!(
                    "Source file at {} contains invalid unicode characters in its name",
                    fpath.to_string_lossy()
                ))?;

                if fname.ends_with(ext) {
                    src_files.push(fpath);
                }
            }
        }
    }

    Ok(src_files)
}
