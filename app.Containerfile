FROM scratch
COPY spin.toml /spin.toml
COPY runtime-config.toml /runtime-config.toml
COPY target/wasm32-wasi/release/*.wasm /wasm32-wasi/
ENTRYPOINT ["/spin.prod.toml"]



# TAG_TIMESTAMP=$(date +%s)
# docker build --platform=wasi/wasm32 --provenance=false -t ghcr.io/prefer-social/kite:$TAG_TIMESTAMP --load -f app.Containerfile .
# docker image tag ghcr.io/prefer-social/kite:$TAG_TIMESTAMP ghcr.io/prefer-social/kite:latest
# docker push ghcr.io/prefer-social/kite:$TAG_TIMESTAMP
# docker push ghcr.io/prefer-social/kite:latest
# docker run --runtime=io.containerd.spin.v2 --platform wasi/wasm32 ghcr.io/prefer-social/kite:latest
