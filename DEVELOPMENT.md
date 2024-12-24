Running command line from source:
``
cargo run -p cli
``

Creating a release build of the command line:
``
cargo build --release -p cli
``

Creating the coverage report:
``
cargo tarpaulin --out Html
``