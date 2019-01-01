docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/Users/johanna/workspace/gender-decoder-rs -w /Users/johanna/workspace/gender-decoder-rs rustlang/rust:nightly cargo build --release
