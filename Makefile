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
	cargo fmt --all -- --check

.PHONY: format
format: ## Format all rust sources
	cargo fmt --all

.PHONY: sqlx-check
sqlx-check: ## Ensure sqlx offline mode metadata is up to date
	cargo sqlx prepare --check -- --lib

.PHONY: sqlx-prepare
sqlx-prepare: ## Generate sqlx offline mode metadata
	cargo sqlx prepare -- --lib

.PHONY: ci
ci: sqlx-check clippy fmt test ## Run all checks as they would run in CI
	@echo "NOTE: tarpaulin was not run since it will cause all targets to recompile."
	@echo "To run it manually, call 'make tarpaulin'."
