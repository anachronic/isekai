FROM rust:1-alpine
RUN apk add musl-dev
RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo build --release
ENTRYPOINT ["/app/target/release/isekai"]
