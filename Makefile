doc_build:
	cargo doc --no-deps --workspace

spin.build:
	spin build

dev.run:
	spin up --build --runtime-config-file runtime-config.toml --env APP_LOG_LEVEL=DEBUG --listen 0.0.0.0:8003

dev.kill:
	pkill spin

run: 
	spin up --build --runtime-config-file runtime-config.toml --env APP_LOG_LEVEL=TRACE --listen 0.0.0.0:8003

kill:
	pkill spin

release:
	HELLO = world
	TAG_TIMESTAMP := $(HELLO) world!
	docker build --platform=wasi/wasm32 --provenance=false -t ghcr.io/prefer-social/kite:$$(TAG_TIMESTAMP) --load -f app.Containerfile .
	docker image tag ghcr.io/prefer-social/kite:$(TAG_TIMESTAMP) ghcr.io/prefer-social/kite:latest
	docker push ghcr.io/prefer-social/kite:$(TAG_TIMESTAMP)
	docker push ghcr.io/prefer-social/kite:latest
	docker run --runtime=io.containerd.spin.v2 --platform wasi/wasm32 ghcr.io/prefer-social/kite:latest

build:
    spin build 

clean:
	cargo clean

cli:
	cargo build --package cli --target x86_64-unknown-linux-gnu	
	cargo run --package cli --target x86_64-unknown-linux-gnu	

all:
	build 
