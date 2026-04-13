use crate::{
    commands::init::ProjectKind,
    project::{ProjectBinSection, ProjectDependenciesSection, ProjectFile, ProjectLanguage, ProjectLibSection, ProjectPackageSection},
};

// Hardcoded template project files

pub const TEMPLATE_GITIGNORE: &str = include_str!("./files/.gitignore_");

const TEMPLATE_JAVA_LIB: &str = include_str!("./files/Lib.java_");
const TEMPLATE_JAVA_BIN: &str = include_str!("./files/Main.java_");

pub fn template_java_code(kind: ProjectKind, package: &str) -> String {
    // Load an appropriate template
    let mut src = match kind {
        ProjectKind::Bin => TEMPLATE_JAVA_BIN,
        ProjectKind::Lib => TEMPLATE_JAVA_LIB,
    }
    .to_owned();

    // Replace its dynamic section with our package
    src = src.replace("%package%", package);

    src
}

pub fn template_config(package: &str, language: ProjectLanguage, kind: ProjectKind) -> ProjectFile {
    
    let (lib, bin) = match kind {
        ProjectKind::Lib => {
            let lib = ProjectLibSection {
                path: "src/Lib.java".into()
            };
            
            (Some(lib), Vec::new())
        },
        ProjectKind::Bin => {
            let bins = vec![
                ProjectBinSection {
                    name: "main".to_owned(),
                    path: "src/Main.java".into()
                }
            ];
            
            (None, bins)
        }
    }; 
    
    ProjectFile {
        package: ProjectPackageSection {
            package: package.to_owned(),
            version: "1.0.0".to_owned(),
            language,
        },

        lib,
        bin,

        // We're making it so an empty section is generated with no dependencies
        dependencies: Some(ProjectDependenciesSection::default()),
        dev_dependencies: None,
    }
}
