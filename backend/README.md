# Crate IT Asset Management Backend

A Rust-based backend service for the IT Asset Management Platform, built with Axum, PostgreSQL, and Neo4j.

## Features

- **RESTful API**: Comprehensive API for CI asset management
- **Graph Database**: Neo4j integration for relationship visualization
- **Authentication**: JWT-based authentication system
- **Audit Logging**: Complete audit trail for all operations
- **Amortization**: Financial valuation and depreciation tracking
- **Import/Export**: CSV-based bulk operations
- **Background Jobs**: Scheduled tasks for maintenance and calculations
- **Rate Limiting**: Built-in protection against abuse
- **Comprehensive Error Handling**: Structured error responses
- **Logging**: Structured logging with tracing

## Technology Stack

- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL with SQLx
- **Graph Database**: Neo4j with neo4rs
- **Authentication**: JWT (jsonwebtoken)
- **Password Hashing**: bcrypt
- **Serialization**: Serde
- **Validation**: validator
- **Logging**: tracing + tracing-subscriber
- **Async Runtime**: Tokio
- **Background Jobs**: cron scheduler

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL 14 or higher
- Neo4j 5 or higher
- Docker (optional)

### Environment Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and configure your environment variables
3. Ensure PostgreSQL and Neo4j are running
4. Run database migrations

### Running the Application

```bash
# Development mode
cargo run --bin server

# Build and run
cargo build --release
./target/release/server
```

### Docker Development

```bash
# Build development image
docker build -f docker/Dockerfile.dev -t crate-backend:dev .

# Run with docker-compose
docker-compose -f docker/docker-compose.yml up
```

## API Documentation

The API follows RESTful conventions and is documented at `/api/v1`. Key endpoints include:

### Authentication
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/register` - User registration
- `GET /api/v1/auth/me` - Get current user
- `POST /api/v1/auth/logout` - Logout

### CI Management
- `GET /api/v1/ci-types` - List CI types
- `POST /api/v1/ci-types` - Create CI type
- `GET /api/v1/ci-assets` - List CI assets
- `POST /api/v1/ci-assets` - Create CI asset
- `GET /api/v1/ci-assets/:id` - Get CI asset
- `PUT /api/v1/ci-assets/:id` - Update CI asset
- `DELETE /api/v1/ci-assets/:id` - Delete CI asset

### Graph Visualization
- `GET /api/v1/graph/data` - Get full graph data
- `GET /api/v1/graph/nodes/:id/neighbors` - Get node neighbors
- `GET /api/v1/graph/search` - Search nodes

### Audit Logging
- `GET /api/v1/audit/logs` - Get audit logs

### Amortization
- `GET /api/v1/amortization/records` - Get valuation records
- `GET /api/v1/amortization/assets/:id/schedule` - Get amortization schedule

### Import/Export
- `POST /api/v1/import/ci-assets` - Import CI assets from CSV
- `GET /api/v1/export/ci-assets` - Export CI assets to CSV

## Configuration

The application uses environment variables for configuration. See `.env.example` for all available options.

### Database Configuration
- PostgreSQL connection settings
- Neo4j connection settings
- Connection pool sizes

### Security Configuration
- JWT secret and expiration
- Password policy
- CORS settings
- Rate limiting

### Logging Configuration
- Log level (debug, info, warn, error)
- Log format (json, pretty)

## Architecture

The application follows a clean architecture pattern:

- **Handlers**: HTTP request/response handling
- **Services**: Business logic layer
- **Repositories**: Data access layer
- **Models**: Data structures and validation
- **Middleware**: Cross-cutting concerns (auth, logging, CORS)
- **Utils**: Reusable utility functions

## Database Schema

### PostgreSQL Tables
- `users` - User accounts
- `ci_types` - CI type definitions
- `ci_assets` - CI asset records
- `relationship_types` - Relationship type definitions
- `audit_log` - Audit trail
- `valuation_records` - Financial valuations
- `amortization_entries` - Depreciation schedules

### Neo4j Graph
- CI asset nodes
- Relationship edges
- Graph constraints and indexes

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Database Migrations
Migrations are run automatically on startup. Manual migration:
```bash
sqlx migrate run --database-url "postgresql://..."
```

## Deployment

### Production Build
```bash
cargo build --release
```

### Docker Production
```bash
docker build -f docker/Dockerfile -t crate-backend:latest .
docker run -p 3000:3000 --env-file .env crate-backend:latest
```

### Environment Variables
All sensitive configuration should be provided via environment variables in production. Never commit secrets to version control.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run tests and linting
6. Submit a pull request

## License

This project is licensed under the MIT License.