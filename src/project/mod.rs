use std::{collections::HashMap, fs, io, path};

use serde::{Serialize, Deserialize};

use crate::{consts::{PROJECT_FILE, PROJECT_TARGET, PROJECT_TMP, TEMP_FILE}, utils::{awrite_file, get_or_make_dir}};

/// Supported project languages
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectLanguage {
    #[serde(rename = "java")]
    Java,

    #[serde(rename = "kotlin")]
    Kotlin
}

/// The [package] section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPackageSection {
    pub package: String,
    pub version: String,
    pub language: ProjectLanguage
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
    pub dependencies: Option<ProjectDependenciesSection>,
    pub dev_dependencies: Option<ProjectDependenciesSection>
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
    awrite_file(
        path.join(PROJECT_FILE), src, get_project_tmp_file(path)?
    )?;

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