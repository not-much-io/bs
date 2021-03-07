.DEFAULT_GOAL:= help

build: ## Build project in debug mode
	@cargo build

format: ## Format code
	@cargo fmt

format-check: ## Check formatting of code
	@cargo fmt -- --check

lint: ## Lint and autofix code
	@cargo fix --allow-dirty

lint-check: ## Check code with linter
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

# TODO: Probably don't have to cargo clean?
package: ## Build and move all binaries to bin dir
	@cargo clean --target x86_64-apple-darwin && \
		cargo clean --target x86_64-unknown-linux-gnu && \
		rm -f bin/x86_64_linux/distrunner && \
		rm -f -r bin/x86_64_linux/tests/* && \
		rm -f bin/x86_64_darwin/distrunner && \
		rm -f -r bin/x86_64_darwin/tests/*
	# Build in brute force parallel, faster than serially
	@make build-x86_64_linux & \
		make build-x86_64_linux-tests & \
		make build-x86_64_darwin & \
		make build-x86_64_darwin-tests
	# Still building in serial to get any error messages in a more readable
	@make build-x86_64_linux && \
		make build-x86_64_darwin && \
		make build-x86_64_linux-tests && \
		make build-x86_64_darwin-tests
	@cp target/x86_64-unknown-linux-gnu/release/distrunner bin/x86_64_linux/
	@cp target/x86_64-apple-darwin/release/distrunner bin/x86_64_darwin/
	@cp target/x86_64-unknown-linux-gnu/debug/deps/distunner-* bin/x86_64_linux/tests && \
		rm -f bin/x86_64_linux/tests/distrunner-*.d
	@cp target/x86_64-apple-darwin/debug/deps/distrunner-* bin/x86_64_darwin/tests && \
		rm -f bin/x86_64_darwin/tests/distrunner-*.d

run-packaged-tests: ## Run packaged tests, usage: run-packaged-tests target=x86_64_linux
	@sh bin/run_all_tests.sh $(target)

it:
	@sh its/run.sh

dbuild-image: ## Build the defined docker image. Usage: make dbuild-image variant=Base|VSCode|CI
	@docker build \
		--file Dockerfile.$(variant) \
		--tag distrunner-$(variant)-image \
		.

dcreate-container: ## Create the defined docker container. Usage: make dcreate-container variant=Base|VSCode|CI
	@docker create \
		--name distrunner-$(variant)-container \
		distrunner-$(variant)-image

dstart-container: ## Start the defined docker container. Usage: make dstart-container variant=Base|VSCode|CI
	@docker start distrunner-$(variant)-container -a

dclean: ## Remove everything associated with the defined dockerfile. Usage: make dclean variant=Base|VSCode|CI
	@docker stop distrunner-$(variant)-container
	@docker rm distrunner-$(variant)-container
	@docker rmi -f distrunner-$(variant)-image

dinit: ## Initialize project, VSCode setup done by VSCode
	@make dbuild-image variant=base

# Source: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
