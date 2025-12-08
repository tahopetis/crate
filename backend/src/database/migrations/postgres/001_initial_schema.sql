-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    is_admin BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

-- CI Types table
CREATE TABLE ci_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    attributes JSONB DEFAULT '{}',
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

-- CI Assets table
CREATE TABLE ci_assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ci_type_id UUID NOT NULL REFERENCES ci_types(id),
    name VARCHAR(255) NOT NULL,
    attributes JSONB DEFAULT '{}',
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL,
    deleted_by UUID REFERENCES users(id)
);

-- Relationship Types table
CREATE TABLE relationship_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    from_ci_type_id UUID REFERENCES ci_types(id),
    to_ci_type_id UUID REFERENCES ci_types(id),
    attributes_schema JSONB DEFAULT '{}',
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

-- Indexes
CREATE INDEX idx_ci_assets_type_id ON ci_assets(ci_type_id);
CREATE INDEX idx_ci_assets_name ON ci_assets(name);
CREATE INDEX idx_ci_assets_created_by ON ci_assets(created_by);
CREATE INDEX idx_ci_types_name ON ci_types(name);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_ci_assets_deleted_at ON ci_assets(deleted_at);
CREATE INDEX idx_ci_types_deleted_at ON ci_types(deleted_at);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ci_types_updated_at BEFORE UPDATE ON ci_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ci_assets_updated_at BEFORE UPDATE ON ci_assets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_relationship_types_updated_at BEFORE UPDATE ON relationship_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();