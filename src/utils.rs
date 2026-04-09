use std::{
    fs, io::{self, Write}, path, process
};

pub fn readln() -> String {
    let mut s = String::with_capacity(32);

    let _ = io::stdin().read_line(&mut s);
    // TODO: This operation can fail

    s
}

/// Print the provided message and return user's response
pub fn prompt(message: &str) -> String {
    print!("{message}");

    let _ = io::stdout().flush();
    
    readln()
}

/// Atomically write a file 
/// 
/// To do that, it needs to know the path for a temporary file, usually on the same disk device.
/// 
/// The purpose of this operation is to be crash safe. User stopping the program mid-execution will not corrupt important
/// configuration files and break the entire project. 
pub fn awrite_file<P1, C, P2>(path: P1, contents: C, temp: P2) -> io::Result<()>
where 
    P1: AsRef<path::Path>,
    P2: AsRef<path::Path>,
    C: AsRef<[u8]> 
{
    fs::write(&temp, contents)?;
    fs::rename(&temp, path)
}

/// Execute a silent command as a child process with no input and output, at the end returning its exit status
pub fn silent_command(commmand: &str, arguments: &[&str]) -> io::Result<process::ExitStatus> {
    process::Command::new(commmand)
        .args(arguments)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()
}

/// Check whether the provided path exists and if not - create it
/// 
/// Note that this doesn't actually check if the file is a directory
pub fn get_or_make_dir(path: &path::Path) -> io::Result<path::PathBuf> {
    if fs::exists(path)? {
        Ok(path.to_owned())
    } else {
        fs::create_dir_all(path)?;
        Ok(path.to_owned())
    }
}