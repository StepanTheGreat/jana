use crate::commands::CommandHandler;

pub struct HelpCommand;
impl CommandHandler for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Show helpful information about the program itself or a particular command"
    }

    fn handle(&mut self, ctx: super::CommandCtx) -> anyhow::Result<()> {
        println!("Available commands:");
        for (name, docs) in ctx.command_docs.iter() {
            println!("- `{name}`: {docs};");
        }

        Ok(())
    }
}
