# Build stage
FROM rust:1.78-buster as builder

WORKDIR /app

# Copy the source code
COPY . .

# Set SQLX_OFFLINE to true if you have a .sqlx directory
ENV SQLX_OFFLINE=true

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/r_uber_bot /app/r_uber_bot

# Copy resources (memes)
COPY --from=builder /app/resources /app/resources

RUN apt-get update && apt install -y openssl ca-certificates sqlite3

CMD ["./r_uber_bot"]
