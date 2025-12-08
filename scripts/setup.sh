#!/bin/bash

# Crate IT Asset Management Platform - Development Setup Script
# This script sets up the complete development environment

set -e

echo "🚀 Setting up Crate IT Asset Management Platform..."

# Check prerequisites
check_prerequisites() {
    echo "📋 Checking prerequisites..."

    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker is not installed. Please install Docker first."
        exit 1
    fi

    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null; then
        echo "❌ Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi

    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        echo "❌ Node.js is not installed. Please install Node.js 18+ first."
        exit 1
    fi

    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust is not installed. Please install Rust first."
        echo "   Visit: https://rustup.rs/"
        exit 1
    fi

    # Check if pnpm is installed
    if ! command -v pnpm &> /dev/null; then
        echo "📦 Installing pnpm..."
        npm install -g pnpm
    fi

    echo "✅ All prerequisites satisfied"
}

# Set up Docker containers
setup_docker() {
    echo "🐳 Setting up Docker containers..."

    # Start databases
    docker-compose up -d postgres neo4j redis

    echo "⏳ Waiting for databases to be ready..."

    # Wait for PostgreSQL
    while ! docker-compose exec -T postgres pg_isready -U dev -d crate_dev; do
        echo "   Waiting for PostgreSQL..."
        sleep 2
    done

    # Wait for Neo4j
    while ! docker-compose exec -T neo4j cypher-shell -u neo4j -p dev123 "RETURN 1" > /dev/null 2>&1; do
        echo "   Waiting for Neo4j..."
        sleep 2
    done

    echo "✅ Docker containers are ready"
}

# Setup backend
setup_backend() {
    echo "🦀 Setting up Rust backend..."

    cd backend

    # Install dependencies
    cargo build

    # Copy environment file
    if [ ! -f .env ]; then
        cp .env.example .env
        echo "📝 Created .env file from example. Please review the configuration."
    fi

    echo "✅ Backend setup complete"
}

# Setup frontend
setup_frontend() {
    echo "⚛️ Setting up Next.js frontend..."

    cd ../frontend

    # Install dependencies
    pnpm install

    # Copy environment file
    if [ ! -f .env.local ]; then
        cp .env.local.example .env.local
        echo "📝 Created .env.local file from example. Please review the configuration."
    fi

    echo "✅ Frontend setup complete"
}

# Initialize database schemas
setup_database() {
    echo "🗄️ Setting up database schemas..."

    # Run PostgreSQL initialization
    echo "   Initializing PostgreSQL..."
    cat ../database/postgres/init.sql | docker-compose exec -T postgres psql -U postgres

    # Run backend migrations
    echo "   Running backend migrations..."
    cd ../backend
    cargo run migrate

    # Run Neo4j initialization
    echo "   Initializing Neo4j..."
    cat ../database/neo4j/init.cypher | docker-compose exec -T neo4j cypher-shell -u neo4j -p dev123

    echo "✅ Database setup complete"
}

# Create development script
create_dev_script() {
    echo "📜 Creating development scripts..."

    cat > dev.sh << 'EOF'
#!/bin/bash

# Development startup script
echo "🚀 Starting Crate development environment..."

# Start databases if not running
docker-compose up -d postgres neo4j redis

# Start backend (in background)
echo "🦀 Starting backend..."
cd backend
cargo run &
BACKEND_PID=$!

# Start frontend
echo "⚛️ Starting frontend..."
cd ../frontend
pnpm dev &
FRONTEND_PID=$!

echo "✅ Development environment started!"
echo "   Frontend: http://localhost:3000"
echo "   Backend:  http://localhost:8080"
echo "   Neo4j:    http://localhost:7474 (neo4j/dev123)"
echo ""
echo "Press Ctrl+C to stop all services"

# Wait for interrupt
trap "echo '🛑 Stopping development environment...'; kill $BACKEND_PID $FRONTEND_PID; exit" INT
wait
EOF

    chmod +x dev.sh
    echo "✅ Development script created: ./dev.sh"
}

# Main execution
main() {
    echo ""
    echo "🏗️  Crate IT Asset Management Platform - Development Setup"
    echo "=========================================================="
    echo ""

    check_prerequisites
    setup_docker
    setup_backend
    setup_frontend
    setup_database
    create_dev_script

    echo ""
    echo "🎉 Setup complete! 🎉"
    echo ""
    echo "Next steps:"
    echo "  1. Review .env files in backend/ and frontend/"
    echo "  2. Run './dev.sh' to start the development environment"
    echo "  3. Open http://localhost:3000 to access the application"
    echo "  4. Open http://localhost:7474 to access Neo4j Browser"
    echo ""
    echo "Default credentials:"
    echo "  • Neo4j: neo4j / dev123"
    echo "  • PostgreSQL: dev / dev123"
    echo ""
}

cd "$(dirname "$0")/.."
main "$@"