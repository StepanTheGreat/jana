use crate::{
    commands::{CommandHandler, init::template::TEMPLATE_GITIGNORE}, 
    consts::{PROJECT_SRC, PROJECT_TARGET}, 
    project::{ProjectLanguage, get_project_tmp_file, is_project_dir, write_project_file}, 
    tools::Git, 
    utils::{awrite_file, prompt}
};

use std::{fs, io, path};

mod template;

use template::{template_java_code, template_config};

/// This simply regex patterns only matches three-layer deep package names:
/// - Three levels, not more, not less: `a.b.c`
/// - Each only containing lowercase ascii letters, digits and underscores
/// 
/// Example:
/// `my.package.com`
const PACKAGE_REGEX: &str = r#"^[a-z0-9_]+\.[a-z0-9_]+\.[a-z0-9_]+$"#;

/// Which default project to create
enum ProjectKind {
    Bin,
    Lib
}

/// Verify if the provided package name is valid
fn package_name_valid(pkg: &str) -> bool {
    let exp = regex::Regex::new(PACKAGE_REGEX).expect("Regex expression is valid");
    exp.is_match(pkg)
}

/// Check if the provided directory path is empty
/// 
/// This can fail due to reasons specified in [std::fs::read_dir]
fn dir_empty(path: &path::Path) -> io::Result<bool> {
    Ok(
        fs::read_dir(path)?.count() == 0
    )
}

pub struct InitCommand;
impl CommandHandler for InitCommand {
    fn name(&self) -> &str {
        "init"
    }

    fn description(&self) -> &str {
        "Initialize a jana project"
    }

    fn params(&self) -> &str {
        "Parameters:
    - `--bin`: create a binary project (default)
    - `--lib`: create a library project
    - `--java`: use a java language (default)
    - `--kotlin`: use kotlin language
    - `--overwrite`: skip validation when creating projects in non-empty directories or directories with an existing jana project
    - `--package=...`: directly provide the desired package name (no quotes)"
    }

    fn handle(&mut self, ctx: super::CommandCtx) -> anyhow::Result<()> {
        let mut args = ctx.args.iter();

        // If the path is not specified - we're going to initialize the project in the current directory
        let path = args.next()
            .cloned()
            .unwrap_or(".".to_owned());
        let path = ctx.cwd.join(path);

        // The kind of a project to create (library or binary) 
        let mut kind = ProjectKind::Bin;
        let mut lang = ProjectLanguage::Java;

        // A package name provided in options
        let mut package = None;
        
        // Whether to overwrite an existing project or folder
        let mut force_overwrite = false;

        // We're going to check each argument and modify our state
        for arg in args {

            match arg.as_str() {
                "--bin" => { kind = ProjectKind::Bin },
                "--lib" => { kind = ProjectKind::Lib },
                "--overwrite" => { force_overwrite = true },

                "--kotlin" => { lang = ProjectLanguage::Kotlin },
                "--java" => { lang = ProjectLanguage::Java },

                pkg if pkg.starts_with("--package=") => {
                    let pkg = pkg.replace("--package=", "");

                    if !package_name_valid(&pkg) {
                        return Err(anyhow::anyhow!("Invalid package name provided: \"{pkg}\""));
                    }

                    package = Some(pkg);
                },

                param => {
                    return Err(anyhow::anyhow!("Unrecognized parameter {param}"));
                }
            }
        }

        // If the directory doesn't exist - create it
        if !fs::exists(&path)? {
            fs::create_dir_all(&path)?;
        }

        match dir_empty(&path) {
            // The directory is empty, everything is good
            Ok(true) => (),

            // The directory is not empty
            Ok(false) => {
                
                // Only prompt if we didn't pass an explicit parameter `--overwrite`
                if !force_overwrite {

                    let resp = prompt("The directory is not empty, do you still want to proceed? (Y) ");
                    if !resp.starts_with("Y") {
                        println!("Aborting...");
                        return Ok(());
                    }
                }
            },

            // An error was encountered
            Err(_) => {
                return Err(anyhow::anyhow!("The provided directory either doesn't exist or is not a directory file."));
            },
        }

        if !force_overwrite && is_project_dir(&path).expect("The directory is safe to access") {
            let resp = prompt("The directory already contains a jana project (Jana.toml). Do you still want to proceed? (Y) ");
            if !resp.starts_with("Y") {
                println!("Aborting...");
                return Ok(());
            }
        }

        // Let's find our package now
        let package = match package {

            // If it was provided in the params - skip
            Some(package) => package,

            // In any other case we're going to loop until the user types in a correct package name
            None => loop {
                let package = prompt("Type the full name of the package (example: org.example.com): ")
                    .trim()
                    .to_owned();

                if package_name_valid(&package) {
                    break package;
                } else {
                    println!("Error: invalid package name.\n");
                }
            }
        };

        // Create neccessary directories within the project (like src and target)
        for dir in [PROJECT_SRC, PROJECT_TARGET] {
            
            if let Err(err) = fs::create_dir(path.join(dir)) {
                
                // If we encounter a permission denied error - we're going to properly notify our user about it.
                if err.kind() == io::ErrorKind::PermissionDenied {
                    return Err(anyhow::anyhow!(err));
               }
            }            
        }
        
        let tmp_file = get_project_tmp_file(&path)?;

        // Create a template java file (either main or lib)
        // TODO: Adapt to different languages
        {
            let file_name = if matches!(kind, ProjectKind::Bin) { "Main.java" } else { "Lib.java" };
            let file_src = template_java_code(kind, &package);
            let file_path = path.join(PROJECT_SRC).join(file_name);

            awrite_file(file_path, file_src, &tmp_file)?;
        }

        // Create the project file 
        write_project_file(&path, &template_config(&package, lang))?;

        // Create a gitignore file
        awrite_file(path.join(".gitignore"), TEMPLATE_GITIGNORE, &tmp_file)?;

        // If git is present - let's create a git repository!
        match Git::get() {
            Ok(git) => git.init(&path),
            Err(_) => println!("No git command found, the created project was initialized without a github repository.")
        }

        Ok(())
    }
}