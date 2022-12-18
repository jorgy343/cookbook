FROM rust:alpine as build

RUN apk add musl-dev

WORKDIR /src
COPY . ./

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine as release

WORKDIR /app

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/rocket_sample* ./

RUN addgroup mygroup && \
    adduser --shell /sbin/nologin --disabled-password --no-create-home --ingroup mygroup myuser

RUN chown myuser:mygroup /app/*
RUN chmod 500 /app/*

USER myuser

EXPOSE 8080
ENTRYPOINT ["/app/rocket_sample_api"]