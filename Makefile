.PHONY: help
help:
	@grep -E '^[a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## Run all tests
	cargo test --locked

.PHONY: tarpaulin
tarpaulin: ## Run tarpaulin
	cargo tarpaulin --ignore-tests

.PHONY: clippy
clippy: ## Run cargo clippy
	cargo clippy --locked -- -D warnings

.PHONY: fmt
fmt: ## Run rustfmt in check mode
	cargo fmt --all --check

.PHONY: format
format: ## Format all rust sources
	cargo fmt --all

.PHONY: ci
ci: clippy fmt test tarpaulin ## Run all checks as they would run in CI
