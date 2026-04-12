use crate::{
    commands::{CommandHandler, build::BuildCommand},
    consts::{PROJECT_CLASSES, PROJECT_TARGET},
    project::read_project_file,
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
        let path = ctx.cwd;

        let config = read_project_file(&path)?;
        let java = Java::get()?;

        let class_path = path.join(PROJECT_TARGET).join(PROJECT_CLASSES);

        // TODO: Handle other languages
        let main_class = config.package.package + ".Main";

        java.run(&class_path, &main_class)?;

        Ok(())
    }
}
