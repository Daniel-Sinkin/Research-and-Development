-   Install the latest stable version of Rust using `rustup`
-   Update to a newer Rust version
-   Open locally installed documentation
-   Write and run a “Hello, world!” program using `rustc` directly
-   Create and run a new project using the conventions of Cargo

Create a new Cargo file named `hello_cargo`, if the current folder is not in a Git repo then it immediately creates a git repo.
```console
cargo new hello_cargo
```

Compiles the code in debug mode (fast compile, slow runtime)
```console
cargo build
```

Compiles the code in release mode (slow compile, fast runtime)
```console
cargo build --release
```

Compile and run the coded
```console
cargo run
```

Compile without creating an executable (faster than build)
```console
cargo check
```

