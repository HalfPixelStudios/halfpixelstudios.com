
default: debug

debug:
    cargo run

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy
