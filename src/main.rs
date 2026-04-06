use std::{env, path, process};

const DEFAULT_COMMAND: &str = "help";

/// Get path to the java program (if present)
/// 
/// It will try to access the JAVA_HOME variable which will 
fn java_bin() -> Option<String> {
    // Either get our `JAVA_HOME`, or just execute java directly
    let java_home = env::var("JAVA_HOME")
        .unwrap_or("java".to_owned());

    // Check if this java home path is a valid executable
    let has_java = process::Command::new(&java_home)
        .arg("--version")
        .stdout(process::Stdio::null())
        .status()
        .is_ok();

    // One nuance is that if the JAVA_PATH is incorrect, but java is present - this will think that java doesn't really exist.
    // This is a niche case, but can be improved in the future. 

    if has_java {
        Some(java_home)
    } else {
        None
    }
}

fn main() {
    // let mut args = env::args();

    // let executable = args.next().expect("Path to the executable is always present");
    // let command = args.next().unwrap_or(DEFAULT_COMMAND.to_owned());

    if java_bin().is_none() {
        println!("No java found. Make sure to add it to `PATH` or set the `JAVA_HOME` variable");
    } else {
        println!("Java path: {}", java_bin().unwrap());
    }
}
