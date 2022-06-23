# File Hierachy

Entry | Code
--- | ---
📁 .cargo/ | Project-local cargo configuration, may contain config.toml.
📁 benches/	| Benchmarks for your crate, run via cargo bench, requires nightly by default.
📁 examples/ | Examples how to use your crate, they see your crate like external user would.
\|- my_example.rs | Individual examples are run like cargo run --example my_example.
📁 src/ | Actual source code for your project.
\|- main.rs | Default entry point for applications, this is what cargo run uses.
\|- lib.rs | Default entry point for libraries. This is where lookup for my_crate::f() starts.
📁 src/bin/ | Place for additional binaries, even in library projects.
\|- extra.rs | Additional binary, run with cargo run --bin extra.
📁 tests/ | Integration tests go here, invoked via cargo test. Unit tests often stay in src/ file.
.rustfmt.toml | In case you want to customize how cargo fmt works.
.clippy.toml | Special configuration for certain clippy lints, utilized via cargo clippy
build.rs | Pre-build script, useful when compiling C / FFI, ...
Cargo.toml | Main project manifest, Defines dependencies, artifacts ...
Cargo.lock | Dependency details for reproducible builds; add to git for apps, not for libs.
rust-toolchain.toml | Define toolchain override (channel, components, targets) for this project.

## crate

rust有自己的规则和约定用来组织模块，比如一个包最多可以有一个库crate，任意多个二进制crate、导入文件夹内的模块的两种约定方式... 知道这些约定，就可以快速了解rust的模块系统。
先把一些术语说明一下：

- 包是cargo的一个功能，当执行cargo new xxxx的时候就是创建了一个包。
- crate是 bin 或者 lib 项目。rust约定在Cargo.toml的同级目录下包含src目录并且包含main.rs文件，就是与包同名的 bin crate，如果包目录中包含src/lib.rs，就是与包同名的 lib crate。包内可以有多crate，多个crates就是一个模块的树形结构。如果一个包内同时包含 src/main.rs 和 src/lib.rs，那么他就有两个 crate，如果想有多个 bin crate，rust约定需要将文件放在src/bin目录下，每个文件就是一个单独的 crate。
- crate root 用来描述如何构建 crate 的文件。比如 src/main.rs 或者 src/lib.rs 就是 crate root。crate root 文件将由 Cargo 传递给 rustc 来实际构建 lib 或者 bin 项目。
- 带有Cargo.toml文件的包用来描述如何构建 crate，一个包可以最多有一个 lib crate，任意多个 bin crate。

## modules

- 使用 mod 关键字来创建新模块，后面紧跟着模块名称
- 模块可以嵌套
- 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
- 所有模块均定义在同一个文件中

## packages、crates、modules

- packages: 通过cargo new 创建；
- crates: 通过cargo new --lib 创建。有根包和子包。即一个根包下可以包含多个子包。
- modules: 通过关键字mod加模块定义

## file rule

To expose them, you need to use either of the following options:

- a file named mod.rs, inside of the utils folder
- a file named utils.rs (same name as the folder), at the same level as the utils folder

Whatever option you choose, the file must then explicitly expose files that should be usable outside of the utils folder.


## Build Script

- https://doc.rust-lang.org/cargo/reference/build-scripts.html

 Placing a file named build.rs in the root of a package will cause Cargo to compile that script and execute it just before building the package.

 Just before a package is built, Cargo will compile a build script into an executable (if it has not already been built). It will then run the script, which may perform any number of tasks. The script may communicate with Cargo by printing specially formatted commands prefixed with cargo: to stdout.

 - https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages

Some Cargo packages that link to system libraries have a naming convention of having a -sys suffix. Any package named foo-sys should provide two major pieces of functionality:

- The library crate should link to the native library libfoo. This will often probe the current system for libfoo before resorting to building from source.
- The library crate should provide declarations for types and functions in libfoo, but not higher-level abstractions.


## Reference

- https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html
- https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
- https://cheats.rs/#project-anatomy

