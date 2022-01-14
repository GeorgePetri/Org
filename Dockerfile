FROM rust:1.57 as build

RUN rustup target add aarch64-unknown-linux-gnu
RUN apt-get update && apt-get -y install gcc-aarch64-linux-gnu

WORKDIR /app
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM --platform=linux/arm64/v8 ubuntu:20.04
ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app
COPY static ./static
COPY --from=build /app/target/aarch64-unknown-linux-gnu/release/org ./

EXPOSE 8000

CMD ["/app/org"]