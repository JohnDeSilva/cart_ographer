RUN_DEV_DIR = run_dev

.PHONY: setup run test lint format typecheck clean coverage run-tui setup-web run-web build-web test-web seed

# Default target
all: setup lint typecheck test

# Install dependencies and sync virtual env
setup:
	uv sync

# Seed demo data into the database
seed:
	mkdir -p $(RUN_DEV_DIR)
	uv run python -m app.seed_data

# Run the FastAPI server in reload/development mode
run:
	mkdir -p $(RUN_DEV_DIR)
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

# Run the Rust TUI client (starts backend, kills it on exit)
run-tui:
	@mkdir -p $(RUN_DEV_DIR)
	@echo "Stopping any existing backend on port 8000..."
	@lsof -ti:8000 2>/dev/null | xargs -r kill 2>/dev/null; sleep 0.5
	@echo "Starting FastAPI backend in the background..."
	@uv run uvicorn app.main:app --host 127.0.0.1 --port 8000 > $(RUN_DEV_DIR)/backend.log 2>&1 & \
	BACKEND_PID=$$!; \
	trap 'echo "Stopping background backend (PID: $$BACKEND_PID)..."; kill $$BACKEND_PID 2>/dev/null || true' EXIT INT TERM; \
	echo "Backend PID: $$BACKEND_PID"; \
	for i in 1 2 3 4 5 6 7 8 9 10; do \
		if curl -s --connect-timeout 1 http://127.0.0.1:8000/ > /dev/null; then \
			echo "Backend is up!"; \
			break; \
		fi; \
		sleep 1; \
	done; \
	cargo run --manifest-path tui_client/Cargo.toml

# Setup web client dependencies
setup-web:
	cd web_client && npm install

# Run web client in development mode (starts backend, kills it on exit)
run-web:
	@mkdir -p $(RUN_DEV_DIR)
	@if [ ! -d "web_client/node_modules" ]; then \
		echo "web_client/node_modules not found. Running setup-web..."; \
		$(MAKE) setup-web; \
	fi
	@echo "Stopping any existing backend on port 8000..."
	@lsof -ti:8000 2>/dev/null | xargs -r kill 2>/dev/null; sleep 0.5
	@echo "Starting FastAPI backend in the background..."
	@uv run uvicorn app.main:app --host 127.0.0.1 --port 8000 > $(RUN_DEV_DIR)/backend.log 2>&1 & \
	BACKEND_PID=$$!; \
	trap 'echo "Stopping background backend (PID: $$BACKEND_PID)..."; kill $$BACKEND_PID 2>/dev/null || true' EXIT INT TERM; \
	echo "Backend PID: $$BACKEND_PID"; \
	for i in 1 2 3 4 5 6 7 8 9 10; do \
		if curl -s --connect-timeout 1 http://127.0.0.1:8000/ > /dev/null; then \
			echo "Backend is up!"; \
			break; \
		fi; \
		sleep 1; \
	done; \
	cd web_client && npm run dev

# Build the web client static assets
build-web:
	@if [ ! -d "web_client/node_modules" ]; then \
		echo "web_client/node_modules not found. Running setup-web..."; \
		$(MAKE) setup-web; \
	fi
	cd web_client && npm run build

# Run web client unit tests
test-web:
	@if [ ! -d "web_client/node_modules" ]; then \
		echo "web_client/node_modules not found. Running setup-web..."; \
		$(MAKE) setup-web; \
	fi
	cd web_client && npm run test

# Clean up build/cache artifacts
clean:
	rm -rf .pytest_cache .mypy_cache .ruff_cache .coverage htmlcov web_client/dist $(RUN_DEV_DIR)
	find . -type d -name "__pycache__" -exec rm -r {} +



