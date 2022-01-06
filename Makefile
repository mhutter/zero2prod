.PHONY: help
help:
	@grep -E '^[a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

ci: ## Run all checks as they would run in CI
	cargo tarpaulin --ignore-tests
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
