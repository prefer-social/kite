FROM debian:testing-slim as builder
WORKDIR /work
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y wget

# For local generated image, it does not ship with Spin binary. 
FROM debian:testing-slim
WORKDIR /app
COPY --from=builder /etc/ca-certificates.conf /etc/ca-certificates.conf
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/share/ca-certificates /usr/share/ca-certificates
COPY --from=builder /usr/local/share/ca-certificates /usr/local/share/ca-certificates
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.3 /usr/lib/x86_64-linux-gnu/libssl.so.3
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.3 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
COPY spin.toml spin.toml
COPY runtime-config.toml runtime-config.toml
CMD ["spin", "up", "--runtime-config-file", "runtime-config.toml"]


