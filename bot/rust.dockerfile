#Build stage
FROM rust:1.78-buster as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release .

RUN apt-get update && apt install -y openssl ca-certificates

CMD ["./r_uber_bot"]