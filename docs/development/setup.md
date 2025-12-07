# Development Setup Guide

This guide will help you set up the development environment for the IT Asset Management Platform (Crate).

## Prerequisites

Before you begin, ensure you have the following installed:

- **Git** - Version control
- **Docker & Docker Compose** - For running databases and services
- **Node.js** (v18 or higher) - For Next.js frontend
- **Rust** (latest stable) - For backend development
- **pnpm** (recommended) - Package manager for frontend

### Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Install Node.js and pnpm

```bash
# Install Node.js (using nvm is recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Install pnpm
npm install -g pnpm

# Verify installations
node --version
pnpm --version
```

## Project Setup

### 1. Clone the Repository

```bash
git clone <repository-url>
cd crates
```

### 2. Start Development Services

The project uses Docker Compose to run the required databases:

```bash
# Start development databases
docker-compose up -d postgres neo4j

# Verify services are running
docker-compose ps
```

### 3. Backend Setup

```bash
# Navigate to backend directory
cd backend

# Copy environment file
cp .env.example .env

# Edit .env with your configuration
nano .env
```

#### Environment Variables (backend/.env)

```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=debug

# Database Configuration
DATABASE_URL=postgresql://dev:dev123@localhost:5432/crate_dev
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=dev123

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-in-production
JWT_EXPIRES_IN=24h
REFRESH_TOKEN_EXPIRES_IN=7d

# Application Configuration
APP_NAME=Crate IT Asset Management
APP_VERSION=1.0.0
APP_URL=http://localhost:8080

# Development Settings
CORS_ORIGINS=http://localhost:3000,http://localhost:3001
ALLOW_REGISTRATION=true

# Background Jobs
SCHEDULER_ENABLED=true
AMORTIZATION_JOB_CRON=0 0 2 * * *  # 2 AM daily

# Import/Export
MAX_IMPORT_SIZE=10MB
IMPORT_BATCH_SIZE=100
```

#### Install Dependencies and Run

```bash
# Install Rust dependencies
cargo build

# Run database migrations
cargo run migrate

# Start development server
cargo run
```

The backend will be available at `http://localhost:8080`

### 4. Frontend Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
pnpm install

# Copy environment file
cp .env.local.example .env.local
```

#### Environment Variables (frontend/.env.local)

```env
# API Configuration
NEXT_PUBLIC_API_URL=http://localhost:8080/api
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws

# Application Configuration
NEXT_PUBLIC_APP_NAME=Crate IT Asset Management
NEXT_PUBLIC_APP_VERSION=1.0.0

# Authentication
NEXT_PUBLIC_SESSION_TIMEOUT=60  # minutes

# Feature Flags
NEXT_PUBLIC_ENABLE_GRAPH_VIZ=true
NEXT_PUBLIC_ENABLE_AUDIT_LOG=true
NEXT_PUBLIC_ENABLE_AMORTIZATION=true

# Development Settings
NEXT_PUBLIC_DEV_MODE=true
NEXT_PUBLIC_DEBUG_API=false
```

#### Run Development Server

```bash
# Start development server
pnpm dev
```

The frontend will be available at `http://localhost:3000`

## Database Setup

### PostgreSQL

The development PostgreSQL instance is already configured in Docker Compose. Access details:

- **Host**: localhost
- **Port**: 5432
- **Database**: crate_dev
- **Username**: dev
- **Password**: dev123

#### Connect to PostgreSQL

```bash
# Using psql
psql -h localhost -p 5432 -U dev -d crate_dev

# Using Docker
docker exec -it crates-postgres-1 psql -U dev -d crate_dev
```

### Neo4j

The development Neo4j instance is configured in Docker Compose:

- **Browser UI**: http://localhost:7474
- **Bolt URI**: bolt://localhost:7687
- **Username**: neo4j
- **Password**: dev123

#### Run Initial Setup

```bash
# Navigate to database directory
cd database

# Initialize Neo4j
cat neo4j/init.cypher | docker exec -i crates-neo4j-1 cypher-shell -u neo4j -p dev123

# Seed PostgreSQL with initial data
cat postgres/seed.sql | docker exec -i crates-postgres-1 psql -U dev -d crate_dev
```

## Development Workflow

### 1. Creating a New Feature

```bash
# Create a new feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Run tests
cd backend && cargo test
cd frontend && pnpm test

# Run linters
cd backend && cargo clippy
cd frontend && pnpm lint

# Commit changes
git add .
git commit -m "feat: add your feature description"

# Push and create PR
git push origin feature/your-feature-name
```

### 2. Database Migrations

#### Backend (PostgreSQL)

```bash
# Create new migration
cd backend

# Create migration file
echo "-- Migration description" > database/migrations/postgres/004_new_migration.sql

# Write your SQL migration

# Run migration
cargo run migrate
```

#### Frontend (Neo4j)

```bash
# Create new Cypher migration
echo "-- Cypher migration description" > database/neo4j/002_new_migration.cypher

# Run migration
cat database/neo4j/002_new_migration.cypher | docker exec -i crates-neo4j-1 cypher-shell -u neo4j -p dev123
```

### 3. Testing

#### Backend Tests

```bash
cd backend

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

#### Frontend Tests

```bash
cd frontend

# Run unit tests
pnpm test

# Run E2E tests
pnpm test:e2e

# Run tests with coverage
pnpm test:coverage
```

### 4. Code Quality

#### Backend

```bash
cd backend

# Format code
cargo fmt

# Check code quality
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

#### Frontend

```bash
cd frontend

# Format code
pnpm format

# Lint code
pnpm lint

# Type check
pnpm type-check

# Check for security vulnerabilities
pnpm audit
```

## Useful Development Commands

### Backend

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run with auto-reload on file changes
cargo install cargo-watch
cargo watch -x run

# Database operations
cargo run migrate          # Run migrations
cargo run migrate revert   # Rollback migration
cargo run migrate reset    # Reset database

# Generate API documentation
cargo doc --no-deps --open
```

### Frontend

```bash
# Development with hot reload
pnpm dev

# Build for production
pnpm build

# Start production server
pnpm start

# Analyze bundle size
pnpm analyze

# Clean build cache
pnpm clean
```

### Docker

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Recreate containers
docker-compose up -d --force-recreate

# Access service shell
docker exec -it <service-name> sh
```

## Troubleshooting

### Common Issues

#### 1. Database Connection Errors

```bash
# Check if databases are running
docker-compose ps

# Restart databases
docker-compose restart postgres neo4j

# Check database logs
docker-compose logs postgres
docker-compose logs neo4j
```

#### 2. Port Conflicts

```bash
# Check what's using a port
lsof -i :3000
lsof -i :8080

# Kill process
kill -9 <PID>

# Or change ports in .env files
```

#### 3. Rust Compilation Errors

```bash
# Clean build cache
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

#### 4. Frontend Build Errors

```bash
# Clear node modules
rm -rf node_modules package-lock.json
pnpm install

# Clear Next.js cache
rm -rf .next
pnpm dev
```

### Getting Help

1. **Check logs**: Always check application logs first
2. **GitHub Issues**: Check existing issues or create a new one
3. **Documentation**: Refer to the project documentation in `/docs`
4. **Team Communication**: Use the team's Slack/Teams channel

## IDE Configuration

### VS Code Extensions

Recommended extensions for development:

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",      // Rust language support
    "bradlc.vscode-tailwindcss",    // Tailwind CSS support
    "esbenp.prettier-vscode",       // Code formatter
    "dbaeumer.vscode-eslint",       // JavaScript/TypeScript linting
    "ms-vscode.vscode-typescript-next", // TypeScript support
    "ms-vscode-remote.remote-containers", // Docker support
    "ms-vscode.vscode-json",        // JSON support
    "redhat.vscode-yaml",          // YAML support
    "ms-vscode.vscode-sql",         // SQL support
    "neo4j.cypher-javascript"       // Cypher query support
  ]
}
```

### VS Code Settings

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true,
    "source.organizeImports": true
  },
  "typescript.preferences.importModuleSpecifier": "relative",
  "tailwindCSS.includeLanguages": {
    "typescript": "javascript",
    "typescriptreact": "html"
  }
}
```

## Production Considerations

When setting up for production:

1. **Security**: Change all default passwords and secrets
2. **Environment**: Use production-specific environment variables
3. **Database**: Set up production databases with proper backups
4. **Performance**: Configure appropriate resource limits
5. **Monitoring**: Set up logging and monitoring
6. **SSL**: Configure HTTPS certificates

## Next Steps

After setting up the development environment:

1. Read the [Project Structure](../implementation/project-structure.md) documentation
2. Review the [API Documentation](../api/endpoints.md)
3. Check the [Database Schema](../implementation/database-schema.md)
4. Explore the [User Guide](../user-guide/getting-started.md)

Happy coding! 🚀