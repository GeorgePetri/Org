FROM rust:1.57 as build

WORKDIR /usr/src/org
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
#FROM alpine:3.15.0

COPY --from=build /usr/local/cargo/bin/org /usr/local/bin/org
COPY --from=build /usr/src/org/static /static

EXPOSE 8000

CMD ["org"]