use std::{collections::HashMap, path::PathBuf};

use crate::commands::{build::BuildCommand, clean::CleanCommand, help::HelpCommand, init::InitCommand, run::RunCommand};

mod help;
mod init;
mod build;
mod run;
mod clean;

/// Context passed to each command
struct CommandCtx<'a> {
    /// Current working directory
    pub cwd: PathBuf,

    /// Arguments passed to the command
    pub args: &'a [String],

    /// A map of command names and their descriptions (useful in help)
    pub command_docs: &'a HashMap<String, String> 
}

impl<'a> Clone for CommandCtx<'a> {
    fn clone(&self) -> Self {
        Self {
            cwd: self.cwd.clone(),
            args: self.args,
            command_docs: self.command_docs
        }
    }
}

/// A command handler... *handles* commands. It's dispatched dynamically and performs various tasks
trait CommandHandler: 'static {
    /// All names by the which this command handler can be called
    fn name(&self) -> &str;

    /// The description of the command
    fn description(&self) -> &str { "" }

    /// The description of command's parameters and arguments
    fn params(&self) -> &str { "" }

    /// A list of commands that must be run before this one
    fn requires(&self) -> Option<&str> { None }

    /// The command handler itself
    fn handle(&mut self, ctx: CommandCtx) -> anyhow::Result<()>;
}

pub struct CommandRegistry {
    command_handlers: HashMap<String, Box<dyn CommandHandler>>,
    command_docs: HashMap<String, String>

}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            command_handlers: HashMap::new(),
            command_docs: HashMap::new()
        }
    }

    /// Add a command
    /// 
    /// # Panics
    /// If you add the same command multiple times
    fn add_command(mut self, handler: impl CommandHandler) -> Self {

        let name = handler.name().to_owned();
        let desc = handler.description().to_owned();
        let handler = Box::new(handler) as Box<dyn CommandHandler>;
        
        assert!(!self.command_handlers.contains_key(&name), "Command {} is already present", name);
        self.command_handlers.insert(name.clone(), handler);
        self.command_docs.insert(name, desc);
        
        self
    }

    pub fn has_command(&self, name: &str) -> bool {
        self.command_handlers.contains_key(name)
    }

    pub fn handle_command(&mut self, command: &str, cwd: PathBuf, args: &[String]) -> anyhow::Result<()> {
        
        // Construct command context
        let ctx = CommandCtx { 
            cwd, 
            args, 
            command_docs: &self.command_docs
        };

        // Check for requirements, and if one exists - run it
        {
            // Get our requirement for the current command
            let requirement = self.command_handlers.get(command)
                .unwrap_or_else(|| panic!("No command {command}"))
                .requires()
                .map(|r| r.to_owned());
    
            // If it's not None
            if let Some(requires) = requirement {
                assert_ne!(requires, command, "A command can't require to execute itself");
    
                // Execute it first
                self.command_handlers.get_mut(&requires)
                    .unwrap_or_else(|| panic!("No command {command}"))
                    .handle(ctx.clone())?;
            }
        }

        // Finally, execute our command
        self.command_handlers.get_mut(command)
            .unwrap_or_else(|| panic!("No command {command}"))
            .handle(ctx)
    }
}

/// Get the default command registry with all commands added
pub fn command_registry() -> CommandRegistry {
    CommandRegistry::new()
        .add_command(HelpCommand)
        .add_command(InitCommand)
        .add_command(BuildCommand)
        .add_command(RunCommand)
        .add_command(CleanCommand)
}