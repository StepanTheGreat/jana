# Jana 
## An experimental, stupidly simple java build system

The purpose of the experiment: create a super minimal and simply build system for java projects (though not limited to java, any project that compiles to
JVM bytecode).
The system is largely inspired by cargo, and thus borrows a lot of ideas from it.

The primary issues it's trying to solve are:
1. Use a simple configurational format (like toml or maybe json)
2. Simplify dependency management (test-only, runtime-only, all and so on)
3. No scripting or DSL. Everything else that should be built on top must be built on top (by scripts, composite tools, etc.)
4. No arbitrary binaries or wrappers, everything is configuration only.
5. Be small, efficient and simple. It should run well on poor hardware.
6. Be developer-friendly.
7. Easy to install/upgrade - either install from github releases, or directly from cargo.

The system must feel intuitive enough for anyone to manage a project, without bloating the system with niche features.

## Licensing
MIT or Apache 2.0, under your choice.
