FROM rust:1.67 as builder
WORKDIR /usr/src/only-in-dreams
COPY . .
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/primes /usr/local/bin/primes
COPY --from=builder /usr/local/cargo/bin/guesser /usr/local/bin/guesser
