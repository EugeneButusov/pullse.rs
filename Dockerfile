FROM rust:1.56.0 as builder
WORKDIR /usr/src/pullse
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pullse /usr/local/bin/pullse
CMD ["pullse"]
