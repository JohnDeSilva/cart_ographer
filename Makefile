.PHONY: setup run test lint format typecheck clean coverage run-tui

# Default target
all: setup lint typecheck test

# Install dependencies and sync virtual env
setup:
	uv sync

# Run the FastAPI server in reload/development mode
run:
	uv run uvicorn app.main:app --reload --host 127.0.0.1 --port 8000

# Run all tests using pytest
test:
	uv run pytest

# Run tests and report coverage
coverage:
	uv run pytest --cov=app --cov-report=term-missing --cov-report=html

# Lint the codebase using Ruff
lint:
	uv run ruff check app tests

# Autoformat code using Ruff
format:
	uv run ruff format app tests

# Run strict type checking with mypy
typecheck:
	uv run mypy app

# Run the Rust TUI client
run-tui:
	cargo run --manifest-path tui_client/Cargo.toml

# Clean up build/cache artifacts
clean:
	rm -rf .pytest_cache .mypy_cache .ruff_cache .coverage htmlcov
	find . -type d -name "__pycache__" -exec rm -r {} +

