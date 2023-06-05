FROM rust AS builder
LABEL maintainer="Carson Gee <x@carsongee.com>" \
  org.label-schema.name="Drone Rust" \
  org.label-schema.vendor="Carson Gee" \
  org.label-schema.schema-version="0.1.0"

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install cargo-make --force && \
  cargo install cargo-nextest --locked && \
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

COPY . /src
WORKDIR /src
RUN cargo make release && \
  cp /src/target/release/drone-rust /bin/ && \
  rm -rf /src
CMD drone-rust
