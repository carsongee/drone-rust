FROM rust AS builder

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install cargo-make --force


COPY . /src
WORKDIR /src

RUN cargo make release

FROM rust
LABEL maintainer="Carson Gee <x@carsongee.com>" \
  org.label-schema.name="Drone Cargo Make" \
  org.label-schema.vendor="Carson Gee" \
  org.label-schema.schema-version="0.1.0"
COPY --from=builder /src/target/release/drone-cargo-make /bin/
CMD drone-cargo-make
