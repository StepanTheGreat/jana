# General overview

## Project structure

The system expects the following project structure:

- `.git` (if git is installed)
- `src`
-   `Main.java` / `Main.kt` (for binaries)
-   `Lib.java` / `Lib.kt` (for libraries)
- `target`
- `Jana.toml` (configuration)
- `Jana.lock` (locked dependencies of the project)
- `.gitignore` (if git is installed)

A `jana` project can only use one sole language: be it `java`, `kotlin`, `scala` or anything that compiles to JVM bytecode.
Support for different languages however largely depends on the build system, which ideally should quickly add these when needed.

**Almost all operations on configuration or save files must be performed atomically to minimize corruption.** 

### src
A `jana` project can only use one language at the same time. Why? It seems like support for multiple languages is mostly a niche feature that
carries a lot of unneccessary verbosity. If you're developing a `java` project, there's no need to constantly go through `src/java/...` every single time.
If you would like to use multiple languages - you can create two different projects with different languages and link them together, since `jana` doesn't 
really care about languages used in dependencies. 

### Jana.toml

This configuration file is largely inspired by `Cargo.toml`; It uses the same syntax, similarly allows declaring dependencies, sub-projects and so on.
There will be however differences, since `java` doesn't have one centralized package repository (thus those should be customizable) and it can run multiple
JVM languages. There are a lot of different nuances, but the main purpose stays the same.

### Java.lock

Almost identical to `Cargo.lock`, but with `java's` ecosystem particularities

### target

All project's output. Everything related to cache, temporary files, class files, jars and so on. 

### Commands

#### Legend:
- Items within parenthesis are variables
- Items that end with `?` are optional


- (no arguments) : runs `help`
- `help` : general help command
- `init (projectPath)? --lib? --bin? --java? --kotlin?` : Initialize a new `java` project. By default it will initialize a `java` binary in the current directory, but, languages and library types can be specified. If a directory doesn't exist - one will be created. If a directory exists and is not empty - user
will be prompted to continue.
- `clean` : Clean up the `target` directory.
- `build` : Build the project. Doesn't run it, simply compiles everything.
- `run` : Build and run the project. As of now can only run `bin` projects.