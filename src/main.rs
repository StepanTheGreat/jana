//! include_str!("../README.md")

use std::env;

use crate::{commands::command_registry, consts::DEFAULT_COMMAND};

mod commands;
mod consts;
mod project;
mod tools;
mod utils;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();

    let _exe = args
        .next()
        .expect("Path to the executable is always present");
    let command = args.next().unwrap_or(DEFAULT_COMMAND.to_owned());
    let args: Vec<String> = args.collect();

    let mut handlers = command_registry();

    if handlers.has_command(&command) {
        handlers.handle_command(&command, std::env::current_dir()?, &args)?;
    } else {
        println!(
            "No command \"{command}\" found. Run `help` to get the entire list of available commands."
        );
        // TODO: Do a close-word suggestion in case user made a type
    }

    Ok(())
}
