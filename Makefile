.PHONY: build build-release clean test run check fmt fmt-check clippy help

# Default target
.DEFAULT_GOAL := help

# Core engine directory
CORE_DIR := core
BINARY_NAME := dojigiri-core

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

build: ## Build the core engine (debug)
	@echo "Building $(BINARY_NAME)..."
	cd $(CORE_DIR) && cargo build

build-release: ## Build the core engine (release)
	@echo "Building $(BINARY_NAME) (release)..."
	cd $(CORE_DIR) && cargo build --release

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	cd $(CORE_DIR) && cargo clean

test: ## Run tests
	@echo "Running tests..."
	cd $(CORE_DIR) && cargo test

run: build ## Build and run the core engine
	@echo "Running $(BINARY_NAME)..."
	cd $(CORE_DIR) && cargo run

check: ## Check code without building
	@echo "Checking code..."
	cd $(CORE_DIR) && cargo check

fmt: ## Format code
	@echo "Formatting code..."
	cd $(CORE_DIR) && cargo fmt

fmt-check: ## Check code formatting
	@echo "Checking code formatting..."
	cd $(CORE_DIR) && cargo fmt -- --check

clippy: ## Run clippy linter
	@echo "Running clippy..."
	cd $(CORE_DIR) && cargo clippy -- -D warnings

