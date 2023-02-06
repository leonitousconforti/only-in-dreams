FROM rust:1.67 as builder
WORKDIR /usr/src/only-in-dreams
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/primes /usr/local/bin/primes
