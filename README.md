# Jana 
## An experimental, stupidly simple java build system

The purpose of the experiment: create a super minimal and simple build system for java projects (though not limited to java, any language that compiles to
JVM bytecode).
The system is largely inspired by cargo and thus borrows a lot of ideas from it.

The primary issues it's trying to solve are:
1. Use a simple configurational format (like toml or maybe json)
2. Simplify dependency management (test-only, runtime-only, all and so on)
3. No scripting or DSL. Everything else that should be built on top must be done so using scripts, composite tools or other means
4. No arbitrary binaries or wrappers, everything is configuration only.
5. Be small, efficient and simple. It should run well on poor hardware.
6. Developer-friendly errors.
7. Easy to install/upgrade - either install from github releases, or directly from cargo.

The system must be super intuitive, while also still flexible enough for most applications.

## Licensing
Under your choice: MIT or Apache 2.0 .
