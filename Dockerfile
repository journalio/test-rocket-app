FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/test-rocket-app
COPY . .
RUN cargo +nightly install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]