FROM debian:stable-slim as builder
WORKDIR /work
RUN apt update -y
RUN apt-get install -y wget
RUN wget -c https://github.com/fermyon/spin/releases/download/canary/spin-canary-static-linux-amd64.tar.gz -O - | tar -xz

#FROM debian:stable-slim
FROM alpine:latest
WORKDIR /app
COPY --from=builder /work/spin /usr/bin/spin
COPY spin.toml spin.toml
COPY runtime-config.toml runtime-config.toml
CMD ["spin", "up", "--runtime-config-file", "runtime-config.toml"]


