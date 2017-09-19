cargo update
cargo build --verbose
cargo test --all
cd cli
cargo build --verbose
cd ../cli-debug
cargo build --verbose
