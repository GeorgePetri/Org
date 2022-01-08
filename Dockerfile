FROM rust:1.57 as build

WORKDIR /app
#COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --release

FROM debian:buster-slim
#FROM alpine:3.15.0

WORKDIR /app
COPY --from=build /app/target/release ./
COPY --from=build /app/static ./static

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/app/org"]