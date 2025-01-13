FROM docker.io/rust:latest

RUN cargo install maturin
RUN rustup target add x86_64-pc-windows-gnu


