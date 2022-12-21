FROM rust:alpine as test

RUN apk add musl-dev

WORKDIR /src
COPY . ./

RUN cargo test