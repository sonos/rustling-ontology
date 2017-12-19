cargo update
cargo build --verbose
cargo test --all
cd cli-debug
cargo build --verbose
cd ../cli
cargo build --verbose
