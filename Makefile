// Makefile — common development tasks for AI-Engine monorepo.
.PHONY: build test check fmt lint clean sdk-build sdk-test

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test --workspace

check:
	cargo check --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace -- -D warnings

clean:
	cargo clean
	rm -rf packages/sdk/dist

sdk-build:
	cd packages/sdk && npm run build

sdk-test:
	cd packages/sdk && npm run typecheck

all: check test sdk-build
