FROM rust:1.91-alpine AS builder

# Build phase dependencies
RUN apk add openssl-dev sqlite-dev musl-dev sqlite-static openssl-libs-static libgcrypt-static

WORKDIR /app

COPY . .

RUN cargo build --release

# Find in current directory, target all subfolder (the intermediate 
# build contents) and unfold to one rm command with `+`.
RUN find ./target/release -maxdepth 1 -type d -not -name "release" -exec rm -rf {} +

# The `runner` phase
FROM scratch

WORKDIR /app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/ ./

CMD ["./vismatch-svc"]
