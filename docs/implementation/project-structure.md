# Project Structure Documentation

## Directory Layout

This document describes the complete directory structure for the IT Asset Management Platform (Crate).

```
crates/
├── backend/                           # Rust API service
│   ├── Cargo.toml                     # Rust workspace configuration
│   ├── Cargo.lock
│   ├── .env.example                   # Environment variables template
│   ├── .gitignore
│   ├── README.md
│   ├── src/
│   │   ├── main.rs                    # Application entry point
│   │   ├── lib.rs                     # Library exports
│   │   ├── config/                    # Configuration management
│   │   │   ├── mod.rs
│   │   │   ├── database.rs            # Database connection configs
│   │   │   └── app.rs                 # Application configuration
│   │   ├── models/                    # Data models and schemas
│   │   │   ├── mod.rs
│   │   │   ├── ci_types.rs            # CI Type data model
│   │   │   ├── ci_lifecycle.rs        # CI Lifecycle model
│   │   │   ├── ci_assets.rs           # CI Asset model
│   │   │   ├── relationship_types.rs  # Relationship types model
│   │   │   ├── audit_log.rs           # Audit log model
│   │   │   ├── valuation.rs           # Amortization/valuation model
│   │   │   └── user.rs                # User model
│   │   ├── handlers/                  # API route handlers
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs                # Authentication endpoints
│   │   │   ├── dashboard.rs           # Dashboard endpoints
│   │   │   ├── ci_management.rs       # CI management endpoints
│   │   │   ├── graph.rs               # Graph visualization endpoints
│   │   │   ├── audit.rs               # Audit log endpoints
│   │   │   ├── amortization.rs        # Amortization endpoints
│   │   │   └── import_export.rs       # Import/Export endpoints
│   │   ├── services/                  # Business logic layer
│   │   │   ├── mod.rs
│   │   │   ├── auth_service.rs        # Authentication business logic
│   │   │   ├── ci_service.rs          # CI management logic
│   │   │   ├── graph_service.rs       # Graph operations logic
│   │   │   ├── audit_service.rs       # Audit logging logic
│   │   │   ├── amortization_service.rs # Amortization calculations
│   │   │   └── import_export.rs       # Import/Export processing
│   │   ├── database/                  # Database access layer
│   │   │   ├── mod.rs
│   │   │   ├── postgres.rs            # PostgreSQL connection
│   │   │   ├── neo4j.rs               # Neo4j connection
│   │   │   ├── migrations/
│   │   │   │   ├── postgres/
│   │   │   │   │   ├── 001_initial_schema.sql
│   │   │   │   │   ├── 002_audit_log.sql
│   │   │   │   │   └── 003_valuation_tables.sql
│   │   │   │   └── neo4j/
│   │   │   │       └── 001_initial_schema.cypher
│   │   │   └── repositories/
│   │   │       ├── mod.rs
│   │   │       ├── ci_repository.rs   # CI data access
│   │   │       ├── audit_repository.rs # Audit data access
│   │   │       ├── valuation_repository.rs # Valuation data access
│   │   │       └── graph_repository.rs # Graph data access
│   │   ├── utils/                     # Utility functions
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs                # Authentication utilities
│   │   │   ├── csv.rs                 # CSV processing utilities
│   │   │   ├── validation.rs          # Data validation
│   │   │   ├── json_diff.rs           # JSON diff calculation
│   │   │   └── date_utils.rs          # Date manipulation utilities
│   │   ├── middleware/                # Axum middleware
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs                # Authentication middleware
│   │   │   ├── cors.rs                # CORS middleware
│   │   │   ├── logging.rs             # Request logging
│   │   │   └── rate_limit.rs          # Rate limiting
│   │   ├── jobs/                      # Background job processing
│   │   │   ├── mod.rs
│   │   │   ├── amortization_job.rs    # Daily amortization job
│   │   │   ├── cleanup_job.rs         # Data cleanup jobs
│   │   │   └── scheduler.rs           # Job scheduler
│   │   └── error/                     # Error handling
│   │       ├── mod.rs
│   │       ├── app_error.rs           # Application error types
│   │       └── response.rs            # Standardized API responses
│   ├── tests/                         # Test files
│   │   ├── mod.rs
│   │   ├── api_tests.rs               # API endpoint tests
│   │   ├── database_tests.rs          # Database integration tests
│   │   ├── service_tests.rs           # Service layer tests
│   │   └── utils_tests.rs             # Utility function tests
│   └── docker/                        # Docker configuration
│       ├── Dockerfile                 # Production Dockerfile
│       ├── Dockerfile.dev             # Development Dockerfile
│       └── docker-compose.yml         # Docker Compose configuration
├── frontend/                          # Next.js application
│   ├── package.json                   # Node.js dependencies
│   ├── package-lock.json
│   ├── next.config.js                 # Next.js configuration
│   ├── tailwind.config.js             # Tailwind CSS configuration
│   ├── tsconfig.json                  # TypeScript configuration
│   ├── .env.local.example             # Environment variables template
│   ├── .gitignore
│   ├── README.md
│   ├── src/
│   │   ├── app/                       # App Router structure
│   │   │   ├── layout.tsx             # Root layout component
│   │   │   ├── page.tsx               # Dashboard page
│   │   │   ├── globals.css            # Global styles
│   │   │   ├── loading.tsx            # Loading component
│   │   │   ├── error.tsx              # Error boundary
│   │   │   ├── not-found.tsx          # 404 page
│   │   │   ├── auth/
│   │   │   │   ├── login/
│   │   │   │   │   └── page.tsx       # Login page
│   │   │   │   └── register/
│   │   │   │       └── page.tsx       # Registration page
│   │   │   ├── ci-management/
│   │   │   │   ├── types/
│   │   │   │   │   └── page.tsx       # CI Types management
│   │   │   │   ├── lifecycles/
│   │   │   │   │   └── page.tsx       # Lifecycles management
│   │   │   │   ├── relationships/
│   │   │   │   │   └── page.tsx       # Relationship types
│   │   │   │   ├── assets/
│   │   │   │   │   └── page.tsx       # CI Assets list
│   │   │   │   └── import/
│   │   │   │       └── page.tsx       # Import functionality
│   │   │   ├── graph/
│   │   │   │   └── page.tsx           # Graph visualization
│   │   │   ├── audit/
│   │   │   │   └── page.tsx           # Audit log viewer
│   │   │   ├── amortization/
│   │   │   │   └── page.tsx           # Amortization dashboard
│   │   │   └── api/                   # Next.js API routes
│   │   │       └── auth/
│   │   │           └── login/
│   │   │               └── route.ts   # Next.js auth API
│   │   ├── components/                # Reusable components
│   │   │   ├── ui/                    # shadcn/ui components
│   │   │   │   ├── button.tsx
│   │   │   │   ├── card.tsx
│   │   │   │   ├── table.tsx
│   │   │   │   ├── dialog.tsx
│   │   │   │   ├── form.tsx
│   │   │   │   ├── input.tsx
│   │   │   │   ├── select.tsx
│   │   │   │   ├── checkbox.tsx
│   │   │   │   ├── tabs.tsx
│   │   │   │   ├── badge.tsx
│   │   │   │   ├── alert.tsx
│   │   │   │   └── toast.tsx
│   │   │   ├── auth/
│   │   │   │   ├── login-form.tsx     # Login form component
│   │   │   │   ├── register-form.tsx  # Registration form
│   │   │   │   └── auth-guard.tsx     # Authentication wrapper
│   │   │   ├── layout/
│   │   │   │   ├── header.tsx         # Header component
│   │   │   │   ├── sidebar.tsx        # Sidebar navigation
│   │   │   │   └── footer.tsx         # Footer component
│   │   │   ├── dashboard/
│   │   │   │   ├── stats-cards.tsx    # Dashboard statistics
│   │   │   │   ├── recent-activity.tsx # Recent activity widget
│   │   │   │   └── top-assets.tsx     # Top valued assets
│   │   │   ├── ci-management/
│   │   │   │   ├── ci-type-form.tsx   # CI Type creation/editing
│   │   │   │   ├── ci-list.tsx        # CI Assets list
│   │   │   │   ├── ci-filters.tsx     # CI filtering controls
│   │   │   │   ├── lifecycle-manager.tsx # Lifecycle management
│   │   │   │   ├── asset-form.tsx     # CI Asset form
│   │   │   │   └── relationship-form.tsx # Relationship form
│   │   │   ├── graph/
│   │   │   │   ├── graph-visualization.tsx # Main graph component
│   │   │   │   ├── graph-controls.tsx # Graph control panel
│   │   │   │   ├── node-details.tsx   # Node information panel
│   │   │   │   └── relationship-details.tsx # Relationship details
│   │   │   ├── audit/
│   │   │   │   ├── audit-table.tsx    # Audit log table
│   │   │   │   ├── audit-filters.tsx  # Audit log filters
│   │   │   │   └── change-details.tsx # Change details modal
│   │   │   ├── amortization/
│   │   │   │   ├── valuation-table.tsx # Valuation history table
│   │   │   │   ├── calculation-history.tsx # Calculation history
│   │   │   │   └── amortization-chart.tsx # Depreciation chart
│   │   │   └── common/
│   │   │       ├── loading.tsx        # Loading spinner
│   │   │       ├── error-boundary.tsx # Error boundary component
│   │   │       ├── pagination.tsx     # Pagination component
│   │   │       ├── search.tsx         # Search component
│   │   │       └── export-button.tsx  # Export functionality
│   │   ├── lib/                       # Utility libraries
│   │   │   ├── auth.ts                # Authentication utilities
│   │   │   ├── api.ts                 # API client configuration
│   │   │   ├── utils.ts               # Common utility functions
│   │   │   ├── constants.ts           # Application constants
│   │   │   ├── types.ts               # Shared TypeScript types
│   │   │   ├── validations.ts         # Form validation schemas
│   │   │   └── formatters.ts          # Data formatting utilities
│   │   ├── hooks/                     # Custom React hooks
│   │   │   ├── use-auth.ts            # Authentication hook
│   │   │   ├── use-api.ts             # API request hook
│   │   │   ├── use-graph.ts           # Graph operations hook
│   │   │   ├── use-local-storage.ts   # Local storage hook
│   │   │   └── use-debounce.ts        # Debounce hook
│   │   ├── store/                     # State management
│   │   │   ├── auth-store.ts          # Authentication state
│   │   │   ├── ci-store.ts            # CI data state
│   │   │   ├── graph-store.ts         # Graph visualization state
│   │   │   └── ui-store.ts            # UI state management
│   │   └── styles/                    # Style files
│   │       └── globals.css            # Global CSS styles
│   ├── public/                        # Public assets
│   │   ├── favicon.ico
│   │   ├── logo.svg
│   │   └── manifest.json
│   └── docker/                        # Docker configuration
│       ├── Dockerfile                 # Production Dockerfile
│       └── Dockerfile.dev             # Development Dockerfile
├── database/                          # Database schemas and seeds
│   ├── postgres/
│   │   ├── init.sql                   # Database initialization
│   │   ├── seed.sql                   # Sample data
│   │   └── backup.sql                 # Backup scripts
│   └── neo4j/
│       ├── init.cypher                # Neo4j initialization
│       └── seed.cypher                # Sample graph data
├── docs/                              # Documentation
│   ├── api/                           # API documentation
│   │   ├── endpoints.md               # API endpoints reference
│   │   ├── authentication.md          # Auth API docs
│   │   └── errors.md                  # Error responses
│   ├── deployment/                    # Deployment guides
│   │   ├── docker.md                  # Docker deployment
│   │   ├── kubernetes.md              # Kubernetes deployment
│   │   └── production.md              # Production setup
│   ├── development/                   # Development guides
│   │   ├── setup.md                   # Development setup
│   │   ├── contributing.md            # Contributing guidelines
│   │   └── testing.md                 # Testing guidelines
│   └── user-guide/                    # User documentation
│       ├── getting-started.md         # User guide intro
│       ├── ci-management.md           # CI management guide
│       └── graph-visualization.md     # Graph usage guide
├── scripts/                           # Development and deployment scripts
│   ├── setup.sh                       # Initial project setup
│   ├── dev.sh                         # Development environment
│   ├── build.sh                       # Build scripts
│   ├── deploy.sh                      # Deployment scripts
│   ├── test.sh                        # Test runner
│   └── backup.sh                      # Database backup
├── .gitignore                         # Git ignore file
├── README.md                          # Project README
├── LICENSE                            # Project license
├── docker-compose.yml                 # Full stack development
├── docker-compose.prod.yml            # Production Docker Compose
└── Makefile                           # Common commands and shortcuts
```

## Key Directories Explanation

### Backend Structure

- **`src/config/`**: Configuration management for databases, authentication, and application settings
- **`src/models/`**: Rust structs representing database entities and API request/response types
- **`src/handlers/`**: HTTP request handlers for API endpoints
- **`src/services/`**: Business logic layer that orchestrates operations between handlers and repositories
- **`src/database/`**: Database connection management, migrations, and data access layer
- **`src/utils/`**: Reusable utility functions for common operations
- **`src/middleware/`**: Axum middleware for auth, CORS, logging, etc.
- **`src/jobs/`**: Background job processing for amortization and cleanup tasks

### Frontend Structure

- **`src/app/`**: Next.js 14 App Router structure with file-based routing
- **`src/components/`**: Reusable React components organized by feature
- **`src/lib/`**: Utility functions, API client, and shared types
- **`src/hooks/`**: Custom React hooks for common operations
- **`src/store/`**: Zustand stores for global state management

### Database Organization

- **`database/postgres/`**: PostgreSQL schema definitions, migrations, and seed data
- **`database/neo4j/`**: Neo4j Cypher scripts for graph database setup

### Documentation

- **`docs/api/`**: Detailed API documentation
- **`docs/deployment/`**: Deployment guides for different environments
- **`docs/development/`**: Development setup and contributing guidelines
- **`docs/user-guide/`**: End-user documentation

## Development Workflow

1. **Local Development**: Use Docker Compose to spin up all services
2. **Database Migrations**: Run PostgreSQL migrations and Neo4j scripts automatically
3. **Hot Reload**: Both frontend and backend support hot reload during development
4. **Testing**: Comprehensive test suite with unit, integration, and E2E tests
5. **Deployment**: Container-based deployment with Docker and optional Kubernetes support