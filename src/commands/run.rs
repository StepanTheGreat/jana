use std::path::PathBuf;

use crate::{
    commands::{CommandHandler, build::BuildCommand},
    consts::{PROJECT_CLASSES, PROJECT_TARGET},
    project::{is_project_dir, read_project_file},
    tools::Java,
};

pub struct RunCommand;
impl CommandHandler for RunCommand {
    fn name(&self) -> &str {
        "run"
    }

    fn description(&self) -> &str {
        "Run the current binary project"
    }

    fn requires(&self) -> Option<&str> {
        Some(BuildCommand.name())
    }

    fn handle(&mut self, ctx: super::CommandCtx) -> anyhow::Result<()> {
        let path = PathBuf::default();

        let run_target = ctx.args.iter().next();

        if !is_project_dir(&path)? {
            return Err(anyhow::anyhow!(
                "Not in a project directory, nothing to run"
            ));
        }

        let config = read_project_file(&path)?;
        let class_path = path.join(PROJECT_TARGET).join(PROJECT_CLASSES);

        if config.bin.is_empty() {
            return Err(anyhow::anyhow!("No runnable files declared in the project"));
        }

        if config.bin.len() > 1 && run_target.is_none() {
            return Err(anyhow::anyhow!("Multiple runnable files are specified, which one should be run?"));
        }

        let run_target_ind = run_target.map(|target| config.bin.iter().position(|b| &b.name == target))
            .unwrap_or(Some(0))
            .ok_or(anyhow::anyhow!("Provided runnable file wasn't found"))?;

        let run_class = config.bin[run_target_ind].path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".java", "");

        let java = Java::get()?;
        java.run(&class_path, &format!("{}.{}", config.package.package, run_class))?;

        Ok(())
    }
}
