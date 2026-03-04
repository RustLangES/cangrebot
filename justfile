default: test

check-fmt:
    cargo fmt --all --check
    
clippy: check-fmt
    cargo clippy --all-targets --all-features -- -D warnings

test: clippy
    cargo test --verbose

