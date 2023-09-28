FROM rust:1-slim-buster AS build

RUN cargo new --bin app
WORKDIR /app
RUN cargo new --lib rinha
RUN mv /app/src/main.rs /app/rinha/src/main.rs

COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY .cargo /app/.cargo
COPY rinha/Cargo.toml /app/rinha/
RUN cargo build --release -p rinha

COPY rinha/src /app/rinha/src
COPY rinha/.sqlx /app/rinha/.sqlx
RUN touch /app/rinha/src/main.rs
RUN cargo build --release -p rinha\

FROM debian:buster-slim

COPY --from=build /app/target/release/rinha /app/rinha

CMD "/app/rinha"
