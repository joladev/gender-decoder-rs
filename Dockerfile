FROM rustlang/rust:nightly AS builder
WORKDIR /opt/app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --release

FROM debian:stretch-slim
WORKDIR /opt/app
COPY --from=builder /opt/app/target/release/gender-decoder .
COPY ./Rocket.toml ./Rocket.toml
COPY ./static ./static
CMD [ "./gender-decoder" ]
