# Developer Guidelines & AI Agent Rules

This document establishes the repository-wide standards, architectural constraints, layout structure, and development workflows for AI agents working on **cartographer** — a multi-client restaurant directory system with three user roles (Admin, Customer, Consumer).

---

## Repository Architecture

The system is structured as three independently developed but tightly integrated components, all communicating over HTTP+JWT:

```
cart_ographer/
├── app/                    # Python FastAPI Backend
│   ├── __init__.py         # Package init
│   ├── main.py             # Routes, JWT auth middleware, startup seeding, static mount
│   ├── crud.py             # All DB transactions — the ONLY place SQLAlchemy writes happen
│   ├── models.py           # SQLAlchemy ORM table definitions (Restaurant, User, UserRole, Favorite)
│   ├── schemas.py          # Pydantic request/response models for serialisation
│   └── database.py         # Engine, SessionLocal, Base, get_db dependency
├── docs/                   # Design documentation
│   └── initial_plan.md     # Original project plan
├── tests/                  # Backend pytest suite
│   ├── __init__.py
│   ├── conftest.py         # In-memory SQLite fixtures, dependency overrides
│   └── test_restaurants.py # Integration tests for all API routes (53 tests)
├── tui_client/             # Rust terminal client (ratatui + crossterm + tokio)
│   └── src/
│       ├── main.rs         # App state, key event loop, async runtime, unit tests
│       ├── api.rs          # All reqwest HTTP calls to the FastAPI backend
│       └── ui.rs           # Widget layouts and ratatui rendering routines
├── web_client/             # React SPA (Vite + TypeScript + Vitest)
│   └── src/
│       ├── App.tsx         # All views, state management, auth flow
│       ├── api.ts          # Fetch wrapper with localStorage JWT token caching
│       ├── index.css       # Glassmorphic stylesheets
│       ├── setupTests.ts   # Global mocks for Vitest
│       └── App.test.tsx    # React Testing Library unit tests
├── Makefile                # All developer commands — always use `make <target>`
├── pyproject.toml          # Python deps (uv), mypy, ruff, pytest, coverage config
└── .agents/AGENTS.md       # This file
```

### Architectural Rules

- **All backend persistence** happens exclusively in `app/crud.py`. Routes in `app/main.py` call CRUD functions — they do **not** interact with the database session directly.
- **All HTTP communication** in the Rust client happens exclusively in `tui_client/src/api.rs`. State mutations are in `main.rs`; visual rendering is in `ui.rs`.
- **All backend HTTP calls** in the web client happen exclusively in `web_client/src/api.ts`. Component logic and UI live in `App.tsx`.
- **Never** bypass these boundaries. If a capability doesn't fit an existing module, create a new, clearly named module rather than breaking the separation.

---

## Code Standards & Style

### 1. Variable Naming — Strict No-Abbreviation Rule

Names must communicate intent without requiring surrounding context. Use full, descriptive terms everywhere — function arguments, loop variables, local bindings, and struct fields alike.

| Do NOT write | Write instead |
|---|---|
| `db`, `conn` | `database`, `database_session`, `connection` |
| `r_id`, `rest` | `restaurant_identifier`, `restaurant` |
| `resp`, `req` | `response`, `request` |
| `usr`, `u` | `user`, `current_user` |
| `tok`, `jwt` | `token`, `access_token`, `jwt_payload` |
| `pwd`, `pw` | `password`, `plain_password`, `hashed_password` |
| `ts`, `dt` | `timestamp`, `datetime_value` |
| `msg`, `err` | `message`, `error_message` |
| `i`, `j`, `k` | `index`, `restaurant_index`, `row_number` |

This rule applies equally to **Python**, **Rust**, and **TypeScript** source files.

### 2. Static Typing — 100% Coverage

#### Python (`app/`)
- Every function, method, and variable must carry a type annotation.
- Run `make typecheck` (strict mypy) before every commit. Zero errors are acceptable.
- Use `Optional[T]` from `typing` rather than bare `T | None` union syntax.
- Pydantic models (`schemas.py`) must define all field types explicitly — no bare `Any` unless unavoidable and explicitly justified with a comment.

#### Rust (`tui_client/`)
- Leverage the type system fully. Avoid `unwrap()` or `expect()` in production paths — use `?` propagation or `match`/`if let` with explicit error handling.
- Represent domain state with enums, not raw strings or integers (e.g., `AppView`, `ActivePanel`, `FocusTarget`).

#### TypeScript (`web_client/`)
- `strict: true` is already configured in `tsconfig.json`. Keep it on.
- Never use `any`. Prefer concrete `interface`/`type` definitions for all API response shapes.
- API response shapes must mirror the Pydantic schemas from the backend. Keep them in sync when the backend schema changes.

### 3. Human-Readable Code

Code is read far more than it is written. Every function and non-obvious block must be immediately clear to a reader unfamiliar with the surrounding context.

- **Functions must do one thing.** A function longer than ~40 lines is a signal to extract sub-functions with descriptive names.
- **Name functions with verbs** that describe the action performed: `fetch_restaurant_by_identifier`, `render_restaurant_list_widget`, `hash_user_password`.
- **Avoid boolean traps.** Prefer named parameters or enums over positional `true`/`false` arguments that obscure intent.
- **Explain the *why*, not the *what*.** Comments should justify non-obvious decisions (e.g., the midnight-crossing logic in `crud.py`) — not narrate code that already reads clearly.
- **No magic numbers or strings.** Named constants or enums must be used for sentinel values, status codes, time limits, and enum variants.

### 4. Code Organisation

- Keep modules small and single-purpose. When a module grows beyond its stated responsibility, split it.
- **Do not** create generic filenames like `helpers.py`, `utils.py`, `misc.rs`, or `common.ts`. Name files after their specific domain (e.g., `password_hashing.py`, `time_range_query.py`).
- Group related constants at the top of the module in a clearly labelled block.
- In Rust, use `mod` boundaries to enforce separation between data, logic, and IO layers.

### 5. Linting & Formatting

| Language | Linter | Formatter | Command |
|---|---|---|---|
| Python | `ruff check` | `ruff format` | `make lint` / `make format` |
| Rust | `cargo clippy` | `cargo fmt` | (run directly from `tui_client/`) |
| TypeScript | ESLint | (Vite default) | `make test-web` (includes lint) |

Linting is non-negotiable. All warnings must be resolved — do not suppress lints without an accompanying explanation comment.

---

## Logging Standards

Logging is treated as a first-class concern. It provides observability into production behaviour without a debugger.

### Python (`app/`) — Standard `logging` Module

Use the standard library `logging` module. **Do not** use `print()` for runtime information in production code.

```python
import logging

logger = logging.getLogger(__name__)
```

Always use the module-level `__name__` logger. Never create a root logger or pass a hardcoded string name.

#### What to Always Log

| Event Category | Level | Example |
|---|---|---|
| Application startup / seeding | `INFO` | `"Seeding default admin user into the database"` |
| Incoming route invocation (complex routes) | `DEBUG` | `"Received request to list restaurants with filters: name=%s, type=%s"` |
| Successful database writes | `INFO` | `"Restaurant '%s' (id=%d) created successfully"` |
| Successful database deletes | `INFO` | `"Restaurant id=%d deleted from the database"` |
| Record updates | `INFO` | `"Restaurant id=%d updated: fields changed=%s"` |
| Authentication failures | `WARNING` | `"Failed login attempt for username: '%s'"` |
| Unexpected errors / exceptions | `ERROR` | Always include `exc_info=True` |
| Password resets | `INFO` | `"Password reset completed for username: '%s'"` |
| Restaurant approval / rejection | `INFO` | `"Restaurant '%s' (id=%d) approved by admin"` |
| Location change requests | `INFO` | `"Location change requested for restaurant id=%d: pending='%s'"` |
| Favorites management | `INFO` | `"Favorite added: consumer_id=%d, restaurant_id=%d"` |

#### What NOT to Log
- Plain-text passwords or tokens at any level — ever.
- Excessively chatty `DEBUG` lines in hot loops (e.g., per-row filter iterations).

### Rust (`tui_client/`)

Use `eprintln!` for structured debug output routed to stderr, or integrate the `tracing` crate if more structured logging is introduced. Log:
- Backend connection errors with the full error message.
- HTTP response status codes for non-2xx responses.
- State transitions between views (e.g., `Login` to `Dashboard`).

### TypeScript (`web_client/`)

Use `console.error()` for caught fetch/network errors. Do **not** use `console.log()` in production paths — strip debug logs before committing. Log:
- Failed API calls with the HTTP status and endpoint.
- Authentication state transitions (login, logout, token expiry).

---

## Testing Standards

Testing is the primary safety net for this codebase. Tests are written **before** implementation. A feature is not done until its tests pass.

### Core Principles

1. **Test-First (TDD)**: Write the test to describe the desired behaviour, watch it fail, then implement to make it pass.
2. **Coverage Threshold**: `pyproject.toml` enforces a hard `fail_under = 90` branch coverage gate. This is a minimum floor — aim higher.
3. **Tests as Documentation**: A well-written test communicates the expected contract of a function better than any comment. Name tests to describe the scenario, not the implementation.

### Python Tests (`tests/`)

#### Test Infrastructure

- **In-memory SQLite**: All tests use an in-memory SQLite database (`sqlite:///:memory:` with `StaticPool`) — never the production `restaurants.db`.
- **FastAPI dependency overrides**: `conftest.py` overrides `get_db` to inject the test session, and `get_current_user`/`get_current_admin` to bypass JWT validation in most tests.
- **Auth integration tests**: When testing real authentication behaviour, **temporarily remove** the dependency overrides (as in `test_user_authentication_flow`) and restore them in a `finally` block.
- The test suite contains **53 tests** with **97.2% branch coverage**.

#### Naming Convention

```python
# Good — describes the scenario
def test_filter_restaurants_by_midnight_crossing_schedule() -> None: ...
def test_customer_role_cannot_create_restaurant() -> None: ...
def test_login_fails_with_incorrect_password() -> None: ...
def test_consumer_only_sees_approved_restaurants_list() -> None: ...

# Bad — describes implementation, not behaviour
def test_crud_function() -> None: ...
def test_route_handler() -> None: ...
```

#### What to Test

Every route handler in `app/main.py` must have tests covering:
- The happy path (expected success case).
- The 404 path (resource not found).
- The 401/403 path (unauthenticated and unauthorised access).
- Edge cases specific to the domain logic (e.g., midnight-crossing `is_open_at` filter, location change approval flow, favorites ownership checks).

Every CRUD function in `app/crud.py` must be exercised through the integration test suite — isolated unit tests for CRUD are only needed for logic that cannot be reached via the API.

#### Mocking & Isolation Rules

- **Never make real network requests in tests.** Use `fastapi.testclient.TestClient`, which routes HTTP requests in-process.
- **Never use real external URLs.** Use only `localhost`, `127.0.0.1`, or `example.invalid` if a URL string is required in a test fixture.
- **Never write to or read from `restaurants.db`** in tests. Only the in-memory engine may be used.

#### Running Tests

```bash
# Run the full pytest suite
make test

# Run with branch coverage report (HTML + terminal)
make coverage
```

### Rust Tests (`tui_client/`)

Unit tests live inside `main.rs` under a `#[cfg(test)]` module. Test UI logic and state machine transitions. Do not make live network calls in tests — mock or stub the API layer.

```bash
cargo test --manifest-path tui_client/Cargo.toml
```

### TypeScript Tests (`web_client/`)

Tests use **Vitest** and **React Testing Library**. All tests live in `web_client/src/`.

- Mock all `fetch()` calls using `vi.stubGlobal` or `global.fetch = vi.fn(...)`.
- Never call the live FastAPI backend from web tests.
- Test user-visible behaviour (what is rendered, what changes on interaction) — not implementation internals.

```bash
make test-web
```

---

## Mandatory Developer Workflow

Every change to this codebase, regardless of size, must follow this sequence. No exceptions.

### Step 1: Understand Before Touching

- Read the relevant module(s) fully before editing.
- Identify all callers of any function you plan to modify.
- Map the impact on existing tests before writing any new code.

### Step 2: Test-First Iteration

1. **Write the test first** — describe the new or corrected behaviour in a test that currently fails.
2. **Implement** the minimum code change in `app/` (or the appropriate client) to make the test pass.
3. **Refine** until `make test` and `make coverage` both pass with >= 90% coverage.

### Step 3: Verification Sequence

Run all of the following after **every change** before committing:

```bash
# Python backend
make test        # pytest — all 53 tests must pass
make coverage    # coverage must remain >= 90%
make lint        # ruff — zero warnings
make typecheck   # mypy strict — zero errors

# Rust TUI (when tui_client/ is modified)
cargo test --manifest-path tui_client/Cargo.toml

# Web client (when web_client/ is modified)
make test-web
```

### Step 4: Documentation Synchronicity

If your change affects any of the following, update the corresponding documentation **in the same commit**:

| Change | Documentation to update |
|---|---|
| New API route or changed request/response shape | `README.md` API section |
| New database column or model field | `README.md` project structure |
| New Makefile target | `README.md` commands section |
| Changed keyboard shortcut in TUI | `README.md` TUI controls section |
| New environment variable or config option | `README.md` prerequisites/config section |
| New user role or RBAC change | Both `README.md` and `.agents/AGENTS.md` domain knowledge |

### Step 5: Commits

Use the **Conventional Commits** specification. Commits must be small, focused, and atomic — one logical change per commit.

```
feat(api): add is_open_at time range filter to GET /restaurants
fix(crud): correct midnight-crossing time comparison logic
test(auth): add integration tests for customer RBAC on write routes
docs(readme): update TUI keyboard controls section
refactor(tui): extract restaurant detail rendering into dedicated function
chore(deps): bump ratatui to 0.29.0
```

**Never** bundle unrelated changes into a single commit. If you find yourself writing "and" in a commit message, split the commit.

---

## Domain Knowledge

### User Roles & RBAC

| Role | Permissions |
|---|---|
| `Admin` | Full CRUD on all restaurants; approve/reject submitted restaurants (`PATCH /restaurants/{id}/approve`); approve/reject location changes (`PATCH /restaurants/{id}/approve-location`); all auth endpoints |
| `Customer` | Submit restaurants for approval (`POST /restaurants/submit`); update own restaurant info (except name); toggle own restaurant open/closed status (`PATCH /restaurants/{id}/status`); list own restaurants (`GET /me/restaurants`); request location changes for Brick & Mortar restaurants |
| `Consumer` | View only approved restaurants (`GET /restaurants`, `GET /restaurants/{id}`); manage personal favorites (`GET/POST /favorites`, `DELETE /favorites/{id}`) |

Default seed accounts created at startup:
- `admin` / `adminpassword` (Admin role)
- `consumer` / `consumerpassword` (Consumer role)

These are seeded in the `on_startup` event in `app/main.py` and must be idempotent (check before inserting).

### Restaurant Model

| Field | Type | Notes |
|---|---|---|
| `id` | `int` | Auto-incrementing primary key |
| `name` | `str` | Indexed; supports partial case-insensitive search |
| `restaurant_type` | `RestaurantType` enum | `Food Stall`, `Food Truck`, `Food Cart`, `Brick and mortar Restaurant` |
| `cuisine_type` | `str` | Free-form cuisine description; supports partial case-insensitive search |
| `location` | `str` | Free-form address string |
| `open_time` | `time` | Stored as SQL `TIME` |
| `close_time` | `time` | Stored as SQL `TIME` |
| `open_status` | `bool` | Manually toggled; defaults `False` |
| `description` | `Optional[str]` | Nullable free-text |
| `menu_items` | `Optional[str]` | Free-form menu items list; supports partial case-insensitive search |
| `is_approved` | `bool` | Admin approval flag; defaults `False` |
| `owner_id` | `Optional[int]` | Foreign key to `users.id`; links Customer owners to their restaurants |
| `pending_location` | `Optional[str]` | Pending new location awaiting admin approval |
| `location_change_pending` | `bool` | Flag indicating a location change is awaiting admin approval |

### Favorite Model

| Field | Type | Notes |
|---|---|---|
| `id` | `int` | Auto-incrementing primary key |
| `consumer_id` | `int` | Foreign key to `users.id` (Consumer role only) |
| `restaurant_id` | `int` | Foreign key to `restaurants.id` |
| | | Unique constraint on `(consumer_id, restaurant_id)` to prevent duplicates |

### Time Filter Edge Case — Midnight Crossing

The `is_open_at` query parameter handles restaurants whose `close_time` is earlier in the day than `open_time` (i.e., they trade past midnight). The SQL logic in `crud.py::get_restaurants` uses an `OR` condition:
- **Normal hours** (`open_time <= close_time`): `open_time <= target <= close_time`
- **Midnight crossing** (`open_time > close_time`): `target >= open_time OR target <= close_time`

This logic is non-trivial. Any changes to it **require** corresponding test cases that exercise boundary conditions at midnight.

### Location Change Flow

When a Customer (restaurant owner) updates the location of a `Brick and mortar Restaurant`, the change is not applied directly. Instead:
1. The new location is stored in `pending_location` and `location_change_pending` is set to `True`.
2. An Admin must approve the change via `PATCH /restaurants/{id}/approve-location`.
3. If approved, `pending_location` overwrites `location` and the pending fields are cleared.
4. If rejected, `pending_location` is cleared and `location` remains unchanged.
5. For `Food Truck` and `Food Cart` types, location changes are applied directly without admin approval.

### Restaurant Approval Flow

1. Customers submit restaurants via `POST /restaurants/submit` with `is_approved = False`.
2. Admins approve or skip via `PATCH /restaurants/{id}/approve`.
3. Only approved restaurants are visible to Consumers.
4. Admin-created restaurants (via `POST /restaurants`) are auto-approved.

### JWT Authentication

- Tokens are signed with `HS256` using the secret defined in `app/main.py`.
- Token expiry is 60 minutes (`ACCESS_TOKEN_EXPIRE_MINUTES`).
- The `sub` claim carries the `username`; the `role` claim carries the user role string.
- `get_current_user`, `get_current_admin`, `get_current_customer`, and `get_current_consumer` are FastAPI dependency functions — use them as `Depends()` arguments on routes, never call them directly.

> **CAUTION**
> The `SECRET_KEY` in `app/main.py` is currently a hardcoded placeholder. Do not commit real credentials. If making this production-ready, move secrets to environment variables and document the required env vars in `README.md`.

---

## Client Feature Parity

The TUI (`tui_client/`) and the Web UI (`web_client/`) are two faces of the same product. A user switching between them should encounter no functional surprise — every capability that exists in one client must exist in the other, unless the medium makes it genuinely impractical (see exceptions below).

### Current Feature Baseline

Both clients must always implement all of the following:

| Feature | Backend endpoint(s) | TUI | Web UI |
|---|---|---|---|
| Login with username & password | `POST /auth/login` |  |  |
| Signup as any role (Admin, Customer, Consumer) | `POST /auth/signup` |  |  |
| Reset own password | `POST /auth/reset-password` |  |  |
| Logout / clear session | (local token discard) |  |  |
| List all restaurants (role-filtered) | `GET /restaurants` |  |  |
| Filter restaurants by name (partial, case-insensitive) | `GET /restaurants?name=` |  |  |
| Filter restaurants by cuisine type | `GET /restaurants?cuisine_type=` |  |  |
| Filter restaurants by menu items | `GET /restaurants?menu_items=` |  |  |
| Filter restaurants by approval status | `GET /restaurants?is_approved=` |  |  |
| View restaurant detail (all fields) | `GET /restaurants/{id}` |  |  |
| Create a restaurant *(Admin only)* | `POST /restaurants` |  |  |
| Submit a restaurant for approval *(Customer only)* | `POST /restaurants/submit` |  |  |
| Edit a restaurant *(Admin)* | `PUT /restaurants/{id}` |  |  |
| Edit own restaurant *(Customer, except name)* | `PUT /restaurants/{id}` |  |  |
| Toggle open/closed status *(Admin)* | `PATCH /restaurants/{id}/status` |  |  |
| Toggle own open/closed status *(Customer)* | `PATCH /restaurants/{id}/status` |  |  |
| Delete a restaurant *(Admin only)* | `DELETE /restaurants/{id}` |  |  |
| Approve/reject submitted restaurant *(Admin only)* | `PATCH /restaurants/{id}/approve` |  |  |
| Approve/reject location change *(Admin only)* | `PATCH /restaurants/{id}/approve-location` |  |  |
| List own restaurants *(Customer only)* | `GET /me/restaurants` |  |  |
| Add a restaurant to favorites *(Consumer only)* | `POST /favorites` |  |  |
| Remove a favorite *(Consumer only)* | `DELETE /favorites/{id}` |  |  |
| List favorites *(Consumer only)* | `GET /favorites` |  |  |

### Adding New Features

When a new endpoint is added to the backend or a new user-facing capability is introduced:

1. **Both clients must be updated in the same PR** unless an explicit exception is granted (see below).
2. **Tests for both clients must be updated** — a new route with only one client exercising it is incomplete work.
3. The feature baseline table above must be updated in this document.

### Acceptable Exceptions

Medium constraints mean some features will look or behave differently — that is expected. What is **not** acceptable is a feature being absent entirely from one client.

| Scenario | Acceptable divergence |
|---|---|
| The Web UI uses a modal form; the TUI uses a full-screen form view | Different presentation, same capability |
| The TUI uses keyboard shortcuts; the Web UI uses buttons | Different interaction model, same action |
| The Web UI displays a data table; the TUI uses a list + detail pane layout | Different layout, same data exposed |
| A filter exists in the Web UI but the TUI has no equivalent search | Missing capability — must be added |
| An Admin action is available in the TUI but absent from the Web UI | Missing capability — must be added |

### RBAC Must Be Mirrored

Role-based access control must be enforced consistently in both clients:
- Admin-only actions (create, delete, approve restaurants, approve locations) must be **hidden or disabled** in the UI when the authenticated user holds the `Customer` or `Consumer` role.
- Customer actions (submit restaurant, edit own, toggle own status) must be **hidden or disabled** for `Admin` and `Consumer` roles.
- Consumer actions (favorites management) must be **hidden or disabled** for `Admin` and `Customer` roles.
- Neither client should rely solely on the backend 403 response as the first line of defence — guard the UI proactively based on the stored `role` claim.

---

## Hard Constraints

These rules must never be violated under any circumstances:

1. **No live network calls in tests** — all HTTP in tests goes through `TestClient` (Python) or mocked fetch (TypeScript).
2. **No writes to `restaurants.db` during test runs** — only the in-memory test engine.
3. **No `print()` statements in `app/`** — use the `logging` module.
4. **No suppressing mypy errors with `# type: ignore`** without a comment explaining why it is unavoidable.
5. **No disabling ruff rules with `# noqa`** without a comment explaining the justification.
6. **No `unwrap()` or `expect()` in non-test Rust code** without a justification comment.
7. **No abbreviations in variable names** — anywhere, in any language.
8. **No committing with failing tests, failing mypy, or failing ruff.**
