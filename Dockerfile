FROM rust:1.38.0 as rust
FROM node:13.0.1-buster

COPY --from=rust /usr/local/rustup /usr/local/rustup
COPY --from=rust /usr/local/cargo /usr/local/cargo

ENV RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH \
  RUST_VERSION=1.38.0 \
  USER=root \
  NODE_PATH=/usr/local/lib/node_modules

RUN npm i -g node-gyp eslint benchmark
RUN node-gyp install

RUN rustup component add rust-src
RUN rustup component add rust-analysis
RUN rustup component add rls
RUN rustup component add clippy
RUN rustup component add rustfmt

RUN apt update && apt upgrade -y && apt install -y llvm lldb clang

RUN cargo install bindgen

WORKDIR /workspaces/napi-rust
