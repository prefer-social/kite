FROM scratch
COPY spin /spin
COPY spin.toml /spin.toml
COPY runtime-config.toml /runtime-config.toml
COPY target/wasm32-wasi/release/*.wasm /target/wasm32-wasi/release/
ENTRYPOINT ["/spin.toml"]