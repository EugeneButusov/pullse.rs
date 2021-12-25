FROM rust:1.56.0 as builder
WORKDIR /usr/src/pullse
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /usr/local/cargo/bin/pullse /usr/local/bin/pullse
CMD ["pullse"]
