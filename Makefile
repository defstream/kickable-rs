BUILD_ARGS = --verbose --release --all-targets --all-features --locked

.PHONY: clean format lint test docs build clippy test lint install docker earth earth/services

default: help

all: clean lint test docs build docker earth/services

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clippy: ## Install clippy
	@rustup -q component add clippy

format: ## Format kickable
	@cargo fmt

check: ## Check kickable
	@cargo check ${BUILD_ARGS}

docs: ## Build cargo documentation
	@cargo doc --no-deps

build: ## Build kickable
	@cargo build ${BUILD_ARGS} --all

test: ## Run kickable tests
	@cargo test ${BUILD_ARGS} --bin kickable

lint: clippy ## Run linting against kickable
	@cargo fmt --all -- --check
	@cargo clippy -- -D warnings

install: ## Install kickable
	@cargo install --path . --bin kickable --debug --force --locked

clean: ## Clean the build artifacts
	@cargo clean
	@rm -f *.profraw

docker: ## Build docker image and tag as defstream/kickable:latest
	@docker build -t defstream/kickable:latest .

earth: ## Build kickable via Earthly
	@earthly --push +kickable

earth/services: ## Build kickable services via Earthly
	@earthly --push +axum
	@earthly --push +gotham
	@earthly --push +graphul
	@earthly --push +poem
	@earthly --push +rocket
	@earthly --push +rouille
	@earthly --push +salvo
	@earthly --push +tonic-client
	@earthly --push +tonic-server
	@earthly --push +trillium
	@earthly --push +viz
	@earthly --push +warp
