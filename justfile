default: check build clippy test

check:
    cargo fmt --all --check

build:
    cargo build

test:
    cargo test --verbose

clippy:
    cargo clippy --all-targets --all-features -- -D warnings