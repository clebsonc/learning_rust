When we create a new project with `cargo new my-project` it creates binary
crate. It is possible to also create a library with the comand `cargo new
--lib`.

The default crate for a binary package is the file `src/main.rs` whilst the
default for a library is `src/lib.rs`.

A bynary crate will always have a `main` function, while a library will only
have accessors to be used. Usually a binary crate is used as a command line,
while a library is used by other libraries and crates.
