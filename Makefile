.PHONY: setup run test lint format typecheck clean coverage run-tui setup-web run-web build-web test-web start-backend-bg

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

# Start backend in the background if not already running on port 8000
start-backend-bg:
	@if ! curl -s --connect-timeout 1 http://127.0.0.1:8000/ > /dev/null; then \
		echo "Starting FastAPI backend in the background..."; \
		uv run uvicorn app.main:app --host 127.0.0.1 --port 8000 > backend.log 2>&1 & \
		echo "Waiting for backend to start..."; \
		for i in {1..10}; do \
			if curl -s --connect-timeout 1 http://127.0.0.1:8000/ > /dev/null; then \
				echo "Backend is up!"; \
				break; \
			fi; \
			sleep 1; \
		done; \
	else \
		echo "FastAPI backend is already running on port 8000."; \
	fi

# Run the Rust TUI client
run-tui: start-backend-bg
	cargo run --manifest-path tui_client/Cargo.toml

# Setup web client dependencies
setup-web:
	cd web_client && npm install

# Run web client in development mode
run-web: start-backend-bg
	cd web_client && npm run dev


# Build the web client static assets
build-web:
	cd web_client && npm run build

# Run web client unit tests
test-web:
	cd web_client && npm run test

# Clean up build/cache artifacts
clean:
	rm -rf .pytest_cache .mypy_cache .ruff_cache .coverage htmlcov web_client/dist
	find . -type d -name "__pycache__" -exec rm -r {} +


