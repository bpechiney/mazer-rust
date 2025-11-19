# Invoke `just` with no args to see help text.
default:
    @just --list

# Format all Rust code using rustfmt.
[group: 'format']
fmt:
    cargo fmt --all

# Check formatting without modifying files. Useful for CI.
[group: 'lint']
check-fmt:
    cargo fmt --all -- --check

# Run clippy in non-strict mode (local dev).
[group: 'lint']
clippy:
    cargo clippy --all-targets

# Remove the target directory.
[group: 'build']
clean:
    cargo clean

# Build a debug binary.
[group: 'build']
build:
    cargo build

# Build a release-optimized binary.
[group: 'build']
release:
    cargo build --release

# Run all tests with output visible (nocapture).
[group: 'test']
test:
    cargo test -- --nocapture

# Run the CLI with any arguments, example:
#   just run --version
[group: 'dev']
[no-exit-message]
run *args:
    cargo run -- {{args}}


# Run all common code quality checks: fmt, clippy, test.
[group: 'check']
check:
    just fmt
    just clippy
    just test
