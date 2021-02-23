.DEFAULT_GOAL:= help

build: ## Build project in debug mode
	@cargo build

format: ## Format code
	@cargo fmt

format-check: ## Check formatting of code
	@cargo fmt -- --check

lint: ## Lint code
	@cargo clippy

test: ## Run tests
	@cargo test

build-x86_64_linux: ## Build for linux
	@cargo build \
		--release \
		--target x86_64-unknown-linux-gnu

build-x86_64_darwin: ## Build for macOS
	@cargo build \
		--release \
		--target x86_64-apple-darwin

build-x86_64_linux-tests: ## Build tests for linux
	@cargo test \
		--no-run \
		--target x86_64-unknown-linux-gnu

build-x86_64_darwin-tests: ## Build tests for linux
	@cargo test \
		--no-run \
		--target x86_64-apple-darwin

package: ## Build and move all binaries to bin dir
	@cargo clean --target x86_64-apple-darwin && \
		cargo clean --target x86_64-unknown-linux-gnu && \
		rm -f bin/x86_64_linux/bs && \
		rm -f -r bin/x86_64_linux/tests/* && \
		rm -f bin/x86_64_darwin/bs && \
		rm -f -r bin/x86_64_darwin/tests/*
	@make build-x86_64_linux && \
		cp target/x86_64-unknown-linux-gnu/release/bs bin/x86_64_linux/
	@make build-x86_64_linux-tests && \
		cp target/x86_64-unknown-linux-gnu/debug/deps/bs-* bin/x86_64_linux/tests && \
		rm -f bin/x86_64_linux/tests/bs-*.d
	@make build-x86_64_darwin && \
		cp target/x86_64-apple-darwin/release/bs bin/x86_64_darwin/
	@make build-x86_64_darwin-tests && \
		cp target/x86_64-apple-darwin/debug/deps/bs-* bin/x86_64_darwin/tests && \
		rm -f bin/x86_64_darwin/tests/bs-*.d

run-packaged-tests: ## Run packaged tests, usage: run-packaged-tests target=x86_64_linux
	@sh bin/run_all_tests.sh $(target)

it:
	@sh integration_tests/run_macos_to_linux.sh

dbuild-image: ## Build the defined docker image. Usage: make dbuild-image variant=Base|VSCode|CI
	@docker build \
		--file Dockerfile.$(variant) \
		--tag bs-$(variant)-image \
		.

dcreate-container: ## Create the defined docker container. Usage: make dcreate-container variant=Base|VSCode|CI
	@docker create \
		--name bs-$(variant)-container \
		bs-$(variant)-image

dstart-container: ## Start the defined docker container. Usage: make dstart-container variant=Base|VSCode|CI
	@docker start bs-$(variant)-container -a

dclean: ## Remove everything associated with the defined dockerfile. Usage: make dclean variant=Base|VSCode|CI
	@docker stop bs-$(variant)-container
	@docker rm bs-$(variant)-container
	@docker rmi -f bs-$(variant)-image

dinit: ## Initialize project, VSCode setup done by VSCode
	@make dbuild-image variant=base

# Source: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
