use std::fs;

use crate::{
    commands::CommandHandler, 
    consts::{PROJECT_CLASSES, PROJECT_SOURCES, PROJECT_TARGET}, 
    project::{get_project_source_files, get_project_target, read_project_file}, 
    tools::Java
};

pub struct BuildCommand;
impl CommandHandler for BuildCommand {
    fn name(&self) -> &str {
        "build"
    }

    fn description(&self) -> &str {
        "Build the project"
    }

    fn handle(&mut self, ctx: super::CommandCtx) -> anyhow::Result<()> {
        let path= ctx.cwd;

        let config = read_project_file(&path)?;
        let java = Java::get()?;

        let lang_ext = config.package.language.ext_str();

        let target = get_project_target(&path)?;

        // Then join it with our target directory:
        // target/class/
        let out_dir = target.join(PROJECT_CLASSES);
        let sources_file = target.join(PROJECT_SOURCES);

        // Write a temporary sources.txt file which lists all our files. We don't care about atomicity here, since this file gets generated
        // every single time. 
        {
            let mut sources = String::new();
            for src_file in get_project_source_files(&path, lang_ext)? {
                sources.push_str(&src_file.to_string_lossy());
                sources.push_str("\n");
            }
            fs::write(&sources_file, sources)?;
        }

        // Compile our files
        java.compile(&sources_file, &out_dir)?;
        
        Ok(())
    }
}