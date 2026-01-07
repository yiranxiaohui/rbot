### 构建阶段
FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 运行阶段：使用 Distroless（含 glibc）
FROM debian:bookworm-slim

# 安装 OpenSSL（libssl3）
RUN apt update && apt install -y --no-install-recommends \
    libssl3 fontconfig ca-certificates&& \
    apt clean && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

# 直接复制二进制（Distroless 已含 glibc 和 ca-certificates）
COPY --from=builder /app/target/release/rbot /app/rbot
COPY --from=builder /app/config.toml /app/rbot

WORKDIR /app

ENTRYPOINT ["./rbot"]