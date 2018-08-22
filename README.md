# Rust Gender Decoder

Inspired by [gender-decoder][gender-decoder] by [@lovedaybrooke][lovedaybrooke]. Check it out if you want a much better implementation.

Naive app that recognizes and lists words that match predefined lists of feminine and masculine words. Built using [Rocket][rocket] and written in [Rust][rust].

## Set up rust and update deps

```
rustup update && cargo update
```

## Run it locally

Runs at localhost:8000 by default.

```
cargo run
```

Requires rust nightly (use [rustup][rustup]).

[gender-decoder]: https://github.com/lovedaybrooke/gender-decoder
[lovedaybrooke]: https://github.com/lovedaybrooke
[rocket]: https://github.com/SergioBenitez/Rocket
[rust]: https://www.rust-lang.org/
[rustup]: https://www.rustup.rs/
