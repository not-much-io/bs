.DEFAULT_GOAL:= help

build: ## Build everything
	@cargo build --release

format: ## Format rust code
	@cargo fmt

format-check: ## Check formatting of rust code
	@cargo fmt -- --check

lint: ## Lint rust code
	@cargo clippy

test: ## Rust rust tests
	@cargo test

cross-compile: ## Cross compile for macOS
	@cargo build \
		--release \
		--target x86_64-apple-darwin

it:
	@sh integration_tests/run_macos_to_linux.sh

ci: ## Run CI quality check process
	make lint && make format-check && make test && make build

ci-macos: ## Run CI quality process specifically for macOS
	make test && make cross-compile

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
