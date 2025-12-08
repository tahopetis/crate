# 🏗️ Crate - IT Asset Management Platform

A next-generation, graph-enabled IT Asset Management Platform designed to evolve into a full Enterprise Architecture tool.

## 🎯 MVP-1 Features

- **Flexible CI Types** - Dynamic schema definition with JSON validation
- **Dynamic Attributes** - JSONB-based flexible attribute storage
- **Configurable Lifecycle & Relationship Types** - Metadata-driven configuration
- **Asset CRUD with Import/Export** - CSV-based bulk operations
- **Asset Relationship Graph** - Neo4j-powered relationship visualization
- **Comprehensive Audit Log** - Change tracking across all entities
- **Dashboard Metrics** - KPIs and operational insights
- **Amortization Engine** - Daily asset value calculations

## 🛠️ Tech Stack

### Backend
- **Rust** + **Axum** - High-performance web framework
- **PostgreSQL** - Primary data storage with JSONB
- **Neo4j** - Graph database for relationships
- **SQLx** - Type-safe database operations
- **JWT** - Token-based authentication

### Frontend
- **Next.js 14** - React framework with App Router
- **TypeScript** - Type-safe development
- **shadcn/ui** - Professional component library
- **Tailwind CSS** - Utility-first styling
- **Zustand** - State management
- **Cytoscape.js** - Graph visualization

### Infrastructure
- **Docker** & **Docker Compose** - Container orchestration
- **Redis** - Caching and session storage

## 🚀 Quick Start

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) latest stable
- [pnpm](https://pnpm.io/) (recommended)

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd crate
   ```

2. **Run the setup script**
   ```bash
   ./scripts/setup.sh
   ```

3. **Start development environment**
   ```bash
   ./dev.sh
   ```

4. **Access the application**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - Neo4j Browser: http://localhost:7474

### Default Credentials

- **Neo4j**: neo4j / dev123
- **PostgreSQL**: dev / dev123

## 📁 Project Structure

```
crate/
├── backend/                    # Rust API service
│   ├── src/
│   │   ├── handlers/           # API route handlers
│   │   ├── services/           # Business logic
│   │   ├── database/           # Database access
│   │   ├── models/             # Data models
│   │   ├── middleware/         # Axum middleware
│   │   └── utils/              # Utilities
│   ├── migrations/             # Database migrations
│   └── Cargo.toml              # Dependencies
├── frontend/                   # Next.js application
│   ├── src/
│   │   ├── app/                # App Router pages
│   │   ├── components/         # Reusable components
│   │   ├── lib/                # Utilities
│   │   ├── hooks/              # Custom hooks
│   │   └── store/              # State management
│   └── package.json            # Dependencies
├── database/                   # Database schemas
│   ├── postgres/               # PostgreSQL scripts
│   └── neo4j/                  # Neo4j scripts
├── scripts/                    # Development scripts
├── docs/                       # Documentation
└── docker-compose.yml          # Development services
```

## 🏃‍♂️ Development

### Manual Setup

If you prefer manual setup:

1. **Start databases**
   ```bash
   docker-compose up -d postgres neo4j redis
   ```

2. **Setup backend**
   ```bash
   cd backend
   cargo build
   cp .env.example .env
   cargo run migrate
   cargo run
   ```

3. **Setup frontend**
   ```bash
   cd frontend
   pnpm install
   cp .env.local.example .env.local
   pnpm dev
   ```

### Testing

```bash
# Backend tests
cd backend && cargo test

# Frontend tests
cd frontend && pnpm test

# E2E tests
cd frontend && pnpm test:e2e
```

### Code Quality

```bash
# Backend
cd backend && cargo fmt && cargo clippy

# Frontend
cd frontend && pnpm lint && pnpm type-check
```

## 📚 Documentation

- [Implementation Plan](./docs/implementation/implementation-plan.md)
- [Project Structure](./docs/implementation/project-structure.md)
- [Database Schema](./docs/implementation/database-schema.md)
- [Development Setup](./docs/development/setup.md)
- [Product Requirements](./docs/plan/prd.md)

## 🔧 Configuration

### Backend Environment (.env)
```env
# Database
DATABASE_URL=postgresql://dev:dev123@localhost:5432/crate_dev
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=dev123

# JWT
JWT_SECRET=your-super-secret-jwt-key

# Server
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### Frontend Environment (.env.local)
```env
# API
NEXT_PUBLIC_API_URL=http://localhost:8080/api

# Features
NEXT_PUBLIC_ENABLE_GRAPH_VIZ=true
NEXT_PUBLIC_ENABLE_AUDIT_LOG=true
```

## 🎨 UI/UX

- **Design System**: shadcn/ui components with Tailwind CSS
- **Theme**: Dark/light mode support
- **Responsive**: Mobile-first design
- **Professional**: Enterprise-grade UI standards

## 📊 API Documentation

### Authentication Endpoints
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/register` - User registration
- `GET /api/v1/auth/me` - Current user info
- `POST /api/v1/auth/logout` - User logout

### CI Management
- `GET /api/v1/ci-types` - List CI types
- `POST /api/v1/ci-types` - Create CI type
- `GET /api/v1/ci-assets` - List CI assets
- `POST /api/v1/ci-assets` - Create CI asset

### Graph Visualization
- `GET /api/v1/graph/data` - Full graph data
- `GET /api/v1/graph/nodes/:id/neighbors` - Node relationships
- `GET /api/v1/graph/search` - Search nodes

## 🔒 Security

- JWT-based authentication
- Role-based access control (Admin, Editor, Viewer)
- Input validation and sanitization
- Rate limiting
- SQL injection prevention
- CORS protection

## 📈 Performance

- **API Response**: < 300ms
- **Graph Rendering**: < 3s for 1000 nodes
- **Data Volume**: 100k+ CIs supported
- **Concurrent Users**: 100+ simultaneous

## 🚢 Deployment

### Docker

```bash
# Build and run all services
docker-compose up --build

# Production deployment
docker-compose -f docker-compose.prod.yml up -d
```

### Production Checklist

- [ ] Update all default passwords and secrets
- [ ] Use production environment variables
- [ ] Set up production databases with backups
- [ ] Configure SSL certificates
- [ ] Set up monitoring and logging
- [ ] Review security settings

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📖 Check the [documentation](./docs/)
- 🐛 [Report issues](https://github.com/your-org/crate/issues)
- 💬 Join our [Discord community](https://discord.gg/crate)

---

Built with ❤️ for modern IT Asset Management