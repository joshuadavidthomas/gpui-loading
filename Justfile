set dotenv-load
set unstable

# List all available commands
[private]
default:
    @just --list --list-submodules

build *ARGS:
    cargo build {{ ARGS }}

check *ARGS:
    cargo check --locked --all-targets --all-features {{ ARGS }}

clean:
    cargo clean

clippy *ARGS:
    cargo clippy --locked --all-targets --all-features --fix --allow-dirty {{ ARGS }} -- -D warnings

rustfmt_channel := `sed -n 's/^channel = "\([^"]*\)"/\1/p' tools/rustfmt/rust-toolchain.toml`

fmt *ARGS:
    cargo "+{{ rustfmt_channel }}" fmt --manifest-path "{{ justfile_directory() }}/Cargo.toml" --all {{ ARGS }}

# cargo-hawk must run on the toolchain it was built against.
# Keep this paired with the Hawk version in mise.toml.
hawk_channel := `sed -n 's/^channel = "\([^"]*\)"/\1/p' tools/hawk/rust-toolchain.toml`

[positional-arguments]
hawk *ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo "+{{ hawk_channel }}" hawk check \
        --manifest-path "{{ justfile_directory() }}/Cargo.toml" \
        --target-dir "{{ justfile_directory() }}/target/hawk" \
        -D warnings "$@"

# run pre-commit on all files
lint *ARGS:
    @just --fmt
    prek run --all-files --show-diff-on-failure --color always {{ ARGS }}

# Open the gallery of every spinner
gallery *ARGS:
    cargo run --release --example gallery -- {{ ARGS }}

test *ARGS:
    cargo test {{ ARGS }}
