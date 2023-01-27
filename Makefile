BUILD_ARGS?=--verbose --release --all-features --locked
CARGO_INSTALL_ARGS?=--path . --bin kickable --debug --force --locked
CARGO_CLIPPY_ARGS?=-- -D warnings
CARGO_FMT_ARGS?=--all -- --check
DOCKER_REPOSITORY?=defstream/kickable
DOCKER_BUILD_ARGS?=-t $(DOCKER_REPOSITORY):latest

.PHONY: clean format lint test docs build clippy test lint install earthly/build earthly/docker earthly/docker/services

default: help

all: clean lint test docs build docker earthly/docker/services

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' ${MAKEFILE_LIST} | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

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
	@cargo test ${BUILD_ARGS}

lint: clippy ## Run linting against kickable
	@cargo fmt ${CARGO_FMT_ARGS}
	@cargo clippy ${CARGO_CLIPPY_ARGS}

install: ## Install kickable
	@cargo install ${CARGO_INSTALL_ARGS}

clean: ## Clean the build artifacts
	@cargo clean
	@rm -f *.profraw
	@rm -rf dist

docker: ## Build docker image and tag as defstream/kickable:latest
	@docker build ${DOCKER_BUILD_ARGS} .


earthly/ci: ## Build cross compiled binaries in docker via Earthly
	@earthly --ci +archive

earthly/build: ## Build cross compiled binaries in docker via Earthly
	@earthly +archive

earthly/docker: ## Build kickable docker app via Earthly
	@earthly --push +kickable

earthly/docker/services: ## Build kickable docker services via Earthly
	@earthly --push +services
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

depot/docker: ## Build kickable docker app via Depot
	@depot build .

depot/docker/cross: ## Build cross compiled binaries in docker via Depot
	@depot build -f Dockerfile.cross .
