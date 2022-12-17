# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
# application
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
# copy configuration
COPY .env.prod .env
COPY --from=builder /app/target/release/ping-client /
CMD ["./ping-client"]