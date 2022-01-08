FROM rust:1.57 as build

RUN rustup target add aarch64-unknown-linux-musl
RUN apt-get update && apt-get -y install gcc-aarch64-linux-gnu

WORKDIR /app
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --release --target aarch64-unknown-linux-musl

FROM --platform=linux/arm64/v8 alpine:3.12

WORKDIR /app
COPY --from=build /app/target/release ./
COPY --from=build /app/static ./static

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/app/org"]