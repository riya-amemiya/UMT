.PHONY: all install format lint typecheck test build clean publish-test publish help

# Default target
all: format lint typecheck test

# Install dependencies
install:
	uv sync

# Format code
format:
	uv run ruff format

# Run lint check
lint:
	uv run ruff check

# Auto-fix lint issues
lint-fix:
	uv run ruff check --fix

# Run type check
typecheck:
	uv run pyright

# Run tests
test:
	uv run pytest

# Build package
build: clean
	uv run --with build python -m build

# Clean build artifacts
clean:
	rm -rf dist/ build/ *.egg-info src/*.egg-info

# Publish to test PyPI
publish-test: build
	uv run --with twine python -m twine upload --repository testpypi dist/*

# Publish to PyPI
publish: build
	uv run --with twine python -m twine upload --repository pypi dist/*

# Show help
help:
	@echo "Available commands:"
	@echo "  make install       - Install dependencies"
	@echo "  make format        - Format code"
	@echo "  make lint          - Run lint check"
	@echo "  make lint-fix      - Auto-fix lint issues"
	@echo "  make typecheck     - Run type check"
	@echo "  make test          - Run tests"
	@echo "  make build         - Build package"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make publish-test  - Publish to test PyPI"
	@echo "  make publish       - Publish to PyPI"
	@echo "  make all           - Run format, lint, typecheck, and test"
