# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Crate** is a next-generation, graph-enabled IT Asset Management Platform built with Rust + Axum backend and Next.js 14 frontend, designed to evolve into a full Enterprise Architecture tool.

### Technology Stack
- **Backend**: Rust + Axum web framework
- **Frontend**: Next.js 14 (App Router) + TypeScript + shadcn/ui
- **Databases**: PostgreSQL (primary data), Neo4j (graph relationships), Redis (caching)
- **State Management**: Zustand (frontend)
- **Graph Visualization**: Cytoscape.js

## Development Commands

### Quick Start
```bash
# First-time setup (runs all setup steps)
./scripts/setup.sh

# Start all services (after setup)
./dev.sh
```

### Backend (Rust)
```bash
cd backend

# Build
cargo build

# Run server (on port 8080)
cargo run

# Run migrations
cargo run migrate

# Run tests
cargo test

# Code quality
cargo fmt        # Format code
cargo clippy     # Lint
```

### Frontend (Next.js)
```bash
cd frontend

# Install dependencies (using pnpm)
pnpm install

# Run dev server (on port 3000)
pnpm dev

# Build for production
pnpm build

# Start production server
pnpm start

# Code quality
pnpm lint        # ESLint
pnpm type-check  # TypeScript validation
```

### Docker Services
```bash
# Start all services (Postgres, Neo4j, Redis)
docker-compose up -d

# Start specific service
docker-compose up -d postgres

# Stop all services
docker-compose down

# View logs
docker-compose logs -f [service-name]
```

### Database Access
- **PostgreSQL**: Port 5432, User: `dev`, Password: `dev123`, Database: `crate_dev`
- **Neo4j Browser**: http://localhost:7474, User: `neo4j`, Password: `dev12345`
- **Redis**: Port 6379

## Architecture

### Backend Architecture (Rust + Axum)

**Entry Point**: `backend/src/main.rs`
- Initializes databases (PostgreSQL + Neo4j)
- Runs migrations via `sqlx::migrate!("./src/database/migrations/postgres")`
- Starts background jobs (amortization, cleanup)
- Creates Axum router with middleware stack
- Binds server to `0.0.0.0:8080`

**Core Modules**:
- `handlers/` - Axum route handlers (API endpoints)
- `services/` - Business logic layer
- `database/` - Database access layer
  - `postgres.rs` - PostgreSQL connection pool
  - `neo4j.rs` - Neo4j graph database connection
  - `repositories/` - Repository pattern for data access
- `models/` - Data structures and types
- `middleware/` - Auth, CORS, rate limiting, logging
- `jobs/` - Background job scheduler (amortization, cleanup)
- `utils/` - Validation, auth utilities
- `error/` - Centralized error handling

**Application State** (`AppState` in `lib.rs`):
- Shared across all handlers via Axum's state extraction
- Contains: `config`, `pg_pool`, `neo4j_pool`, `database` (repositories), `rate_limiter`
- Cloneable (uses Arc internally for shared resources)

**Database Layer**:
- **Repository Pattern**: All database operations go through repositories
- **Dual Database**: PostgreSQL for relational data, Neo4j for graph relationships
- **Migrations**: SQLx migrations in `backend/src/database/migrations/postgres/`
- **Connection Pools**: Managed by `PgPool` and `Neo4jPool`

### Frontend Architecture (Next.js 14)

**App Router Structure** (`frontend/src/app/`):
- Uses Next.js 14 App Router (not Pages Router)
- `/auth/login`, `/auth/register` - Authentication pages
- `/ci-management/types`, `/ci-management/assets` - CI management
- `/graph` - Graph visualization
- `/audit` - Audit log viewer
- `/amortization` - Asset valuation tracking

**State Management**:
- **Zustand stores** in `frontend/src/store/`:
  - `auth-store.ts` - Authentication state (persisted to localStorage as `auth-storage`)
  - `ci-store.ts` - CI management state
  - `ui-store.ts` - UI state
- Persistence: Auth state is persisted via Zustand middleware

**API Client** (`frontend/src/lib/api.ts`):
- Centralized API client class (`apiClient`)
- Auto-handles JWT token from localStorage (`auth-storage` key)
- Auto-redirects to `/auth/login` on 401 responses
- Predefined endpoints in `apiEndpoints` object
- Base URL: `NEXT_PUBLIC_API_URL` env var (defaults to `http://localhost:3000/api/v1`)

**Component Structure**:
- `components/ui/` - shadcn/ui components (button, dialog, form, etc.)
- `components/layout/` - Layout components (header, sidebar, main-layout)
- `components/auth/` - Auth-specific components (login/register forms)
- `components/ci/` - CI management components

### API Architecture

**Authentication**:
- JWT-based authentication
- Token stored in localStorage (frontend) and validated via middleware (backend)
- Protected routes require `Authorization: Bearer <token>` header
- Auth middleware in `backend/src/middleware/auth.rs`

**API Endpoints** (`/api/v1` prefix):
- **Public**: `/auth/login`, `/auth/register`, `/auth/logout`, `/health`
- **Protected** (require auth):
  - CI Types: `GET/POST /ci-types`, `GET/PUT/DELETE /ci-types/:id`
  - CI Assets: `GET/POST /ci-assets`, `GET/PUT/DELETE /ci-assets/:id`
  - Lifecycle: `GET/POST /lifecycle-types`, `/lifecycle-states`, etc.
  - Relationships: `GET/POST /relationship-types`, etc.
  - Graph: `GET /graph/data`, `GET /graph/nodes/:id/neighbors`, `GET /graph/search`
  - Audit: `GET /audit/logs`
  - Amortization: `GET /amortization/records`, `GET /amortization/assets/:id/schedule`
  - Import/Export: `POST /import/ci-assets`, `GET /export/ci-assets`

**Middleware Stack** (applied in order):
1. TraceLayer (HTTP tracing)
2. CORS middleware
3. Logging middleware
4. Rate limiting (100 requests/min)
5. Auth middleware (protected routes only)

### Database Schema

**PostgreSQL Tables**:
- `users` - User accounts
- `ci_types` - Configuration Item type definitions with JSON schemas
- `ci_assets` - Actual CI instances with JSONB attributes
- `lifecycle_types`, `lifecycle_states` - Lifecycle definitions
- `relationship_types` - Relationship type definitions
- `audit_log` - Change tracking
- `valuation_records`, `valuation_daily_snapshots` - Amortization data

**Neo4j Graph**:
- Mirrors CI assets as nodes
- Relationship types as edges
- Cypher queries for graph traversal and visualization

**Migration System**:
- SQLx migrations in `backend/src/database/migrations/postgres/`
- Run automatically on server startup via `sqlx::migrate!()`
- Neo4j initialization scripts in `database/neo4j/`

## Important Patterns

### Adding New API Endpoints

1. **Handler** (`backend/src/handlers/`): Create handler function with Axum extractors
2. **Service** (`backend/src/services/`): Implement business logic
3. **Repository** (`backend/src/database/repositories/`): Add database operations
4. **Router** (`backend/src/main.rs`): Register route in `create_app()`
5. **Frontend API** (`frontend/src/lib/api.ts`): Add endpoint to `apiEndpoints`

### Adding Database Migrations

```bash
# Create new migration file
cd backend/src/database/migrations/postgres
# Create numbered file: 00X_description.sql
# Migrations run automatically on server startup
```

### Error Handling

- Backend: Use `AppError` type from `backend/src/error/mod.rs`
- Frontend: API client throws errors, catch in components/hooks

### Authentication Flow

1. User logs in via `POST /api/v1/auth/login`
2. Backend returns JWT token
3. Frontend stores in Zustand store (persisted to localStorage)
4. API client automatically includes `Authorization` header
5. Backend middleware validates token on protected routes

### Background Jobs

- Implemented in `backend/src/jobs/`
- Scheduler in `jobs/scheduler.rs`
- Jobs: `amortization_job.rs` (daily valuation), `cleanup_job.rs`
- Started on server initialization

## Configuration

### Backend Environment Variables (.env)
```
DATABASE_URL=postgresql://dev:dev123@localhost:5432/crate_dev
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=dev12345
JWT_SECRET=your-super-secret-jwt-key
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### Frontend Environment Variables (.env.local)
```
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_ENABLE_GRAPH_VIZ=true
NEXT_PUBLIC_ENABLE_AUDIT_LOG=true
```

## Common Patterns

### Adding a New CI Type Field
1. CI Types use JSON Schema validation stored in PostgreSQL JSONB
2. No migration needed - schemas are dynamic
3. Update frontend form components to handle new fields

### Querying Graph Relationships
- Use `GraphRepository` in `backend/src/database/repositories/graph_repository.rs`
- Neo4j Cypher queries for traversal
- Results serialized to JSON for frontend

### State Management (Frontend)
- Use Zustand stores for global state
- Local component state for UI-only state
- Persist authentication state only

## Testing Strategy

### Backend
- Unit tests: In each module (`#[cfg(test)]` modules)
- Integration tests: Test full API endpoints
- Run: `cd backend && cargo test`

### Frontend
- Component tests (when implemented)
- E2E tests (when implemented)
- Run: `cd frontend && pnpm test`

## Deployment

Production deployment uses Docker Compose:
```bash
docker-compose -f docker-compose.prod.yml up -d
```

Ensure you update default credentials and secrets before production deployment.

## Graph Visualization

- Uses Cytoscape.js (`frontend/src/app/graph/`)
- Data fetched from `/api/v1/graph/data`
- Supports node search and neighbor exploration
- Layout algorithms for visualization

## Key Files to Reference

- **Backend Entry**: `backend/src/main.rs`
- **Backend State**: `backend/src/lib.rs`
- **API Routes**: `backend/src/main.rs` (in `create_app()`)
- **Frontend API**: `frontend/src/lib/api.ts`
- **Auth Store**: `frontend/src/store/auth-store.ts`
- **Database Migrations**: `backend/src/database/migrations/postgres/`
- **Environment Examples**: `backend/.env.example`, `frontend/.env.local.example`
