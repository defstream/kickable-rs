BUILD_ARGS = --verbose --release --all-targets --all-features --locked

default: help

all: clean lint test build

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clippy: ## Install clippy
	@rustup -q component add clippy

check: ## Check kickable
	@cargo check ${BUILD_ARGS}

build: ## Build kickable
	@cargo build ${BUILD_ARGS} --bin kickable

test: ## Run kickable tests
	@cargo test ${BUILD_ARGS}

lint: clippy ## Run linting against kickable
	@cargo fmt --all -- --check
	@cargo clippy -- -D warnings

install: ## Install kickable
	@$cargo install --path . --debug --force

clean: ## Clean the build artifacts
	@cargo clean