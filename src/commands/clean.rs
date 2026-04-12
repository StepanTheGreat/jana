use std::fs;

use crate::{commands::CommandHandler, consts::PROJECT_TARGET, project::is_project_dir};

pub struct CleanCommand;
impl CommandHandler for CleanCommand {
    fn name(&self) -> &str {
        "clean"
    }

    fn description(&self) -> &str {
        "Clean target directory"
    }

    fn handle(&mut self, ctx: super::CommandCtx) -> anyhow::Result<()> {
        let path = ctx.cwd;

        if !is_project_dir(&path)? {
            return Err(anyhow::anyhow!(
                "Not in a project directory, nothing to clean"
            ));
        }

        // TODO: Handle permission errors
        let _ = fs::remove_dir_all(path.join(PROJECT_TARGET));

        Ok(())
    }
}
