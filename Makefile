build:
	cargo doc --no-deps --workspace
	cargo build

spin.build:
	spin build

dev.run:
	spin up --build --runtime-config-file runtime-config.toml --env APP_LOG_LEVEL=DEBUG --listen 0.0.0.0:8003

dev.kill:
	pkill spin

run: 
	spin up --build --runtime-config-file runtime-config.toml --env APP_LOG_LEVEL=DEBUG --listen 0.0.0.0:8003

kill:
	pkill spin


clean:
	cargo clean

all:
	build 
