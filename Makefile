BUILD_ARGS?=--verbose --release --all-features --locked
CARGO_CLIPPY_ARGS?=-- -D warnings
CARGO_FMT_ARGS?=--all -- --check
CARGO_INSTALL_ARGS?=--path . --bin kickable --debug --force --locked
DOCKER_BUILD_ARGS?=-t $(DOCKER_REPOSITORY):latest
DOCKER_REPOSITORY?=defstream/kickable

.PHONY: clean format lint test docs build clippy test lint sonar/scan install earthly/build earthly/docker earthly/docker/services docker help all check

default: help

all: clean format lint test docs sonar/scan build docker earthly/docker/services ## Runs all the things 😅

help: ## Print this help message 🙋🏽
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' ${MAKEFILE_LIST} | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clippy: ## Install clippy 🖇️
	@rustup -q component add clippy

format: ## Format kickable ℹ
	@cargo fmt

check: ## Check kickable ✓
	@cargo check ${BUILD_ARGS}
	@./scripts/sonar-scan.sh

docs: ## Build cargo documentation 📑
	@cargo doc --no-deps

build: ## Build kickable 🛠️
	@cargo build ${BUILD_ARGS} --all

test: ## Run kickable tests 🧪
	@cargo test ${BUILD_ARGS}

lint: clippy check ## Run linting against kickable 🏃🏽
	@cargo fmt ${CARGO_FMT_ARGS}
	@cargo clippy ${CARGO_CLIPPY_ARGS}

install: ## Install kickable 💻
	@cargo install ${CARGO_INSTALL_ARGS}

clean: ## Clean the build artifacts 🧹
	@cargo clean
	@rm -f *.profraw

docker: ## Build docker image and tag as kickable/kickable:latest 🐳
	@docker build -f docker/Dockerfile ${DOCKER_BUILD_ARGS} .

earthly/ci: ## Build cross compiled binaries in docker via Earthly
	@earthly --ci +archive

earthly/build: ## Build cross compiled binaries in docker via Earthly
	@earthly --ci --output +archive

earthly/docker: ## Build kickable docker app via Earthly
	@earthly --ci --push --output +kickable

earthly/docker/services: ## Build kickable docker services via Earthly
	@earthly --ci --push +services

depot/builder: ## Build cross compiled binaries in docker via Depot
	@depot build --platform linux/amd64,linux/arm64 -f docker/Dockerfile.builder -t kickable/builder .

depot/docker: depot/builder ## Build kickable docker app via Depot
	@depot build -f docker/Dockerfile .

depot/docker/cross: depot/builder ## Build cross compiled binaries in docker via Depot
	@depot build -f docker/Dockerfile.cross .

score/build: ## Build kickable services via Score
	@score-compose run -f ./score/axum.yaml -o ./score/axum.compose.yaml
	@score-compose run -f ./score/gotham.yaml -o ./score/gotham.compose.yaml
	@score-compose run -f ./score/graphul.yaml -o ./score/graphul.compose.yaml
	@score-compose run -f ./score/poem.yaml -o ./score/poem.compose.yaml
	@score-compose run -f ./score/rocket.yaml -o ./score/rocket.compose.yaml
	@score-compose run -f ./score/rouille.yaml -o ./score/rouille.compose.yaml
	@score-compose run -f ./score/tonic.yaml -o ./score/tonic.compose.yaml
	@score-compose run -f ./score/viz.yaml -o ./score/viz.compose.yaml
	@score-compose run -f ./score/warp.yaml -o ./score/warp.compose.yaml

score/up: ## Launch the score kickable services
	@docker compose \
		-f score/axum.compose.yaml \
		-f score/gotham.compose.yaml \
		-f score/graphul.compose.yaml \
		-f score/poem.compose.yaml \
		-f score/rocket.compose.yaml \
		-f score/rouille.compose.yaml \
		-f score/tonic.compose.yaml \
		-f score/viz.compose.yaml \
		-f score/warp.compose.yaml \
		up

cross/build: ## Build cross compiled binaries in docker via Cross
	@cargo build --release --all-features --locked --target aarch64-apple-darwin
	@cargo build --release --all-features --locked --target aarch64-unknown-linux-musl
	@cargo build --release --all-features --locked --target x86_64-apple-darwin
	@RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc' cargo build --release --all-features --locked --target x86_64-pc-windows-gnu
	@RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --release --all-features --locked --target x86_64-unknown-linux-musl


sonar/scan: ## Scan kickable with SonarQube
	@docker run \
        --rm \
        --net host \
        -e SONAR_HOST_URL=${SONAR_HOST_URL} \
        -v ${PWD}:/usr/src  \
        sonarsource/sonar-scanner-cli \
   		-Dsonar.projectKey=kickable \
    	-Dsonar.sources=/usr/src \
		-Dsonar.host.url=${SONAR_HOST_URL} \
    	-Dsonar.token=${SONAR_TOKEN}