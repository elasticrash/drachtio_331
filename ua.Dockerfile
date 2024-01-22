FROM rust:1-bookworm

COPY ./ttg/src ./src
COPY ./ttg/Cargo.toml ./Cargo.toml

RUN cargo build
