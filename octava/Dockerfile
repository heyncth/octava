FROM rust:1.97-alpine AS build
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.22
RUN adduser -D -u 10001 octava
COPY --from=build /app/target/release/octava /usr/local/bin/octava
USER octava
WORKDIR /work
ENTRYPOINT ["octava"]
