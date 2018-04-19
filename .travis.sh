run_test() {
    cd $1
    cargo build --verbose
    cargo test
    cd ..
}

cargo update
cargo build --verbose
cargo test
run_test cli-debug
run_test moment
run_test values
run_test json-utils