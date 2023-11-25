FROM rust:1.70-slim-bookworm as builder

WORKDIR /usr/src/filaco

# Cache deps
RUN cargo init --bin .
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm -r src/

COPY src src

# Release build
RUN rm ./target/release/deps/filaco*
RUN cargo build --release

FROM debian:bookworm-slim as final

COPY --from=builder /usr/src/filaco/target/release/filaco /usr/local/bin/filaco

CMD ["filaco"]