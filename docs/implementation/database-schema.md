# Database Schema Documentation

## Overview

This document describes the complete database schema for the IT Asset Management Platform (Crate). The system uses two databases:
- **PostgreSQL**: Primary data storage with JSONB for flexible attributes
- **Neo4j**: Graph database for storing and querying CI relationships

## PostgreSQL Schema

### 1. Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'editor', 'viewer')),
    is_active BOOLEAN DEFAULT true,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_active ON users(is_active);
```

### 2. CI Types Table

```sql
CREATE TABLE ci_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    schema_json JSONB NOT NULL DEFAULT '{}',
    icon VARCHAR(100), -- Icon name or URL
    color VARCHAR(7), -- Hex color code
    is_active BOOLEAN DEFAULT true,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_ci_types_name ON ci_types(name);
CREATE INDEX idx_ci_types_active ON ci_types(is_active);
CREATE INDEX idx_ci_types_created_by ON ci_types(created_by);
```

**Schema JSON Structure:**
```json
{
  "fields": {
    "hostname": {
      "type": "string",
      "required": true,
      "min_length": 3,
      "max_length": 100
    },
    "ip_address": {
      "type": "string",
      "required": true,
      "pattern": "^[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}$"
    },
    "cpu_cores": {
      "type": "number",
      "required": true,
      "min": 1,
      "max": 128
    },
    "memory_gb": {
      "type": "number",
      "required": true,
      "min": 1
    },
    "operating_system": {
      "type": "enum",
      "required": true,
      "options": ["Linux", "Windows", "macOS"]
    },
    "is_virtual": {
      "type": "boolean",
      "required": false,
      "default": false
    },
    "purchase_date": {
      "type": "date",
      "required": false
    },
    "warranty_expiry": {
      "type": "date",
      "required": false
    }
  },
  "display_fields": ["hostname", "ip_address", "operating_system"],
  "unique_fields": ["hostname"]
}
```

### 3. CI Lifecycles Table

```sql
CREATE TABLE ci_lifecycles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    "order" INTEGER NOT NULL,
    color VARCHAR(7), -- Hex color code for UI
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_ci_lifecycles_name ON ci_lifecycles(name);
CREATE INDEX idx_ci_lifecycles_order ON ci_lifecycles("order");
CREATE INDEX idx_ci_lifecycles_active ON ci_lifecycles(is_active);
```

### 4. Relationship Types Table

```sql
CREATE TABLE relationship_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    is_bidirectional BOOLEAN DEFAULT false,
    color VARCHAR(7), -- Hex color code for graph visualization
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_relationship_types_name ON relationship_types(name);
CREATE INDEX idx_relationship_types_active ON relationship_types(is_active);
```

### 5. Configuration Items (CIs) Table

```sql
CREATE TABLE cis (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ci_type_id UUID NOT NULL REFERENCES ci_types(id),
    name VARCHAR(255) NOT NULL,
    attributes_json JSONB NOT NULL DEFAULT '{}',
    lifecycle_id UUID REFERENCES ci_lifecycles(id),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_cis_type ON cis(ci_type_id);
CREATE INDEX idx_cis_lifecycle ON cis(lifecycle_id);
CREATE INDEX idx_cis_name ON cis(name);
CREATE INDEX idx_cis_created_by ON cis(created_by);
CREATE INDEX idx_cis_updated_by ON cis(updated_by);
CREATE INDEX idx_cis_attributes ON cis(attributes_json) USING GIN;
CREATE INDEX idx_cis_created_at ON cis(created_at);
CREATE INDEX idx_cis_updated_at ON cis(updated_at);

-- Full-text search index
CREATE INDEX idx_cis_search ON cis USING GIN (
  to_tsvector('english', name || ' ' || COALESCE(attributes_json::text, ''))
);
```

**Attributes JSON Example:**
```json
{
  "hostname": "web-server-01",
  "ip_address": "192.168.1.100",
  "cpu_cores": 8,
  "memory_gb": 16,
  "operating_system": "Linux",
  "is_virtual": true,
  "purchase_date": "2023-01-15",
  "warranty_expiry": "2025-01-15",
  "cost": 5000.00,
  "department": "IT Infrastructure",
  "location": "Data Center A",
  "owner": "john.doe@company.com"
}
```

### 6. Audit Log Table

```sql
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    action VARCHAR(100) NOT NULL, -- 'create', 'update', 'delete', 'import', etc.
    entity_type VARCHAR(100) NOT NULL, -- 'ci', 'ci_type', 'lifecycle', etc.
    entity_id UUID NOT NULL,
    entity_name VARCHAR(255), -- Human-readable name of the entity
    before_json JSONB, -- Complete entity state before change
    after_json JSONB, -- Complete entity state after change
    changes_json JSONB, -- JSON diff for efficient storage
    ip_address INET,
    user_agent TEXT,
    batch_id UUID, -- For grouping related changes (e.g., bulk import)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_audit_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_user ON audit_log(user_id);
CREATE INDEX idx_audit_action ON audit_log(action);
CREATE INDEX idx_audit_created_at ON audit_log(created_at);
CREATE INDEX idx_audit_batch ON audit_log(batch_id);
```

### 7. Import Jobs Table

```sql
CREATE TABLE import_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100), -- 'text/csv', 'application/json', etc.
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    total_rows INTEGER,
    processed_rows INTEGER DEFAULT 0,
    successful_rows INTEGER DEFAULT 0,
    failed_rows INTEGER DEFAULT 0,
    errors JSONB DEFAULT '[]', -- Array of error messages with row numbers
    warnings JSONB DEFAULT '[]', -- Array of warning messages
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_import_jobs_user ON import_jobs(user_id);
CREATE INDEX idx_import_jobs_status ON import_jobs(status);
CREATE INDEX idx_import_jobs_created_at ON import_jobs(created_at);
```

### 8. CI Valuation Daily Table (Amortization)

```sql
CREATE TABLE ci_valuation_daily (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ci_id UUID NOT NULL REFERENCES cis(id),
    date DATE NOT NULL,
    purchase_cost DECIMAL(15,2) NOT NULL,
    useful_life_days INTEGER NOT NULL,
    daily_depreciation DECIMAL(15,2) NOT NULL,
    accumulated_depreciation DECIMAL(15,2) NOT NULL,
    current_value DECIMAL(15,2) NOT NULL,
    is_fully_depreciated BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(ci_id, date)
);

-- Indexes
CREATE INDEX idx_valuation_ci_id ON ci_valuation_daily(ci_id);
CREATE INDEX idx_valuation_date ON ci_valuation_daily(date);
CREATE INDEX idx_valuation_ci_date ON ci_valuation_daily(ci_id, date);
CREATE INDEX idx_valuation_current_value ON ci_valuation_daily(current_value);
CREATE INDEX idx_valuation_depreciated ON ci_valuation_daily(is_fully_depreciated);
```

### 9. System Settings Table

```sql
CREATE TABLE system_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(255) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT false, -- Whether frontend can access this setting
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_system_settings_key ON system_settings(key);
CREATE INDEX idx_system_settings_public ON system_settings(is_public);
```

### 10. User Sessions Table

```sql
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    refresh_token_hash VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_used_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_user_sessions_user ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token ON user_sessions(token_hash);
CREATE INDEX idx_user_sessions_expires ON user_sessions(expires_at);
CREATE INDEX idx_user_sessions_active ON user_sessions(is_active);
```

## Neo4j Schema

### 1. Node Labels

```cypher
-- CI Nodes
CREATE CONSTRAINT ci_id_unique IF NOT EXISTS FOR (c:CI) REQUIRE c.id IS UNIQUE;
CREATE INDEX ci_name_index IF NOT EXISTS FOR (c:CI) ON (c.name);
CREATE INDEX ci_type_index IF NOT EXISTS FOR (c:CI) ON (c.ci_type_name);
CREATE INDEX ci_lifecycle_index IF NOT EXISTS FOR (c:CI) ON (c.lifecycle_name);

-- CI Type Nodes (for metadata)
CREATE CONSTRAINT ci_type_name_unique IF NOT EXISTS FOR (t:CIType) REQUIRE t.name IS UNIQUE;

-- User Nodes (for tracking relationships)
CREATE CONSTRAINT user_id_unique IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE;
```

### 2. Relationship Types

```cypher
-- Relationship type constraints
CREATE CONSTRAINT rel_type_name_unique IF NOT EXISTS FOR (r:RelType) REQUIRE r.name IS UNIQUE;

-- Standard relationships between CIs
(:CI)-[:DEPENDS_ON {type: string, created_by: uuid, created_at: datetime}]->(:CI)
(:CI)-[:CONNECTS_TO {type: string, created_by: uuid, created_at: datetime}]->(:CI)
(:CI)-[:HOSTS {type: string, created_by: uuid, created_at: datetime}]->(:CI)
(:CI)-[:RUNS_ON {type: string, created_by: uuid, created_at: datetime}]->(:CI)
(:CI)-[:RELATED_TO {type: string, created_by: uuid, created_at: datetime}]->(:CI)

-- Metadata relationships
(:CI)-[:HAS_TYPE]->(:CIType)
(:User)-[:CREATED]->(:CI)
(:User)-[:MODIFIED]->(:CI)
```

### 3. Graph Data Model

```cypher
-- Create or update CI node
MERGE (ci:CI {id: $ci_id})
SET ci.name = $ci_name,
    ci.ci_type_name = $ci_type_name,
    ci.lifecycle_name = $lifecycle_name,
    ci.attributes = $attributes,
    ci.updated_at = datetime()
RETURN ci;

-- Create relationship between CIs
MATCH (source:CI {id: $source_id}),
      (target:CI {id: $target_id})
MERGE (source)-[r:RELATED {
    type: $relationship_type,
    name: $relationship_name,
    created_by: $user_id,
    created_at: datetime()
}]->(target)
RETURN r;

-- Get neighbors of a CI
MATCH (ci:CI {id: $ci_id})-[r]-(neighbor:CI)
RETURN ci, r, neighbor
ORDER BY neighbor.name;

-- Get shortest path between two CIs
MATCH path = shortestPath(
    (start:CI {id: $start_id})-[*]-(end:CI {id: $end_id})
)
RETURN path;
```

## Database Functions and Triggers

### 1. Updated At Trigger

```sql
-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply to tables that need updated_at tracking
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ci_types_updated_at BEFORE UPDATE ON ci_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_cis_updated_at BEFORE UPDATE ON cis
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_system_settings_updated_at BEFORE UPDATE ON system_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### 2. Audit Log Trigger

```sql
-- Function to create audit log entries
CREATE OR REPLACE FUNCTION create_audit_log()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_log (
            user_id, action, entity_type, entity_id, entity_name,
            before_json, after_json, created_at
        ) VALUES (
            COALESCE(current_setting('app.current_user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID),
            'delete', TG_TABLE_NAME, OLD.id, OLD.name,
            row_to_json(OLD), NULL, NOW()
        );
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_log (
            user_id, action, entity_type, entity_id, entity_name,
            before_json, after_json, changes_json, created_at
        ) VALUES (
            COALESCE(current_setting('app.current_user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID),
            'update', TG_TABLE_NAME, NEW.id, NEW.name,
            row_to_json(OLD), row_to_json(NEW),
            CASE
                WHEN row_to_json(OLD) = row_to_json(NEW) THEN NULL
                ELSE json_build_object('changes',
                    json_agg(json_build_object('field', key, 'old', old_val, 'new', new_val))
                )
            END,
            NOW()
        );
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_log (
            user_id, action, entity_type, entity_id, entity_name,
            before_json, after_json, created_at
        ) VALUES (
            COALESCE(current_setting('app.current_user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID),
            'create', TG_TABLE_NAME, NEW.id, NEW.name,
            NULL, row_to_json(NEW), NOW()
        );
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Apply audit triggers
CREATE TRIGGER audit_ci_types_trigger
    AFTER INSERT OR UPDATE OR DELETE ON ci_types
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

CREATE TRIGGER audit_cis_trigger
    AFTER INSERT OR UPDATE OR DELETE ON cis
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

CREATE TRIGGER audit_lifecycles_trigger
    AFTER INSERT OR UPDATE OR DELETE ON ci_lifecycles
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

CREATE TRIGGER audit_relationship_types_trigger
    AFTER INSERT OR UPDATE OR DELETE ON relationship_types
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();
```

### 3. JSON Schema Validation Function

```sql
-- Function to validate JSON against schema
CREATE OR REPLACE FUNCTION validate_ci_attributes(ci_type_id UUID, attributes JSONB)
RETURNS BOOLEAN AS $$
DECLARE
    schema_def JSONB;
    field_def JSONB;
    field_name TEXT;
    field_value JSONB;
    field_type TEXT;
    is_required BOOLEAN;
BEGIN
    -- Get the schema definition
    SELECT schema_json INTO schema_def
    FROM ci_types
    WHERE id = ci_type_id;

    IF schema_def IS NULL THEN
        RAISE EXCEPTION 'CI type not found';
    END IF;

    -- Validate each field in the schema
    FOR field_name, field_def IN SELECT * FROM jsonb_each_text(schema_def->'fields')
    LOOP
        field_type := (schema_def->'fields'->field_name->>'type');
        is_required := COALESCE((schema_def->'fields'->field_name->>'required')::BOOLEAN, false);
        field_value := attributes->field_name;

        -- Check required fields
        IF is_required AND field_value IS NULL THEN
            RAISE EXCEPTION 'Required field % is missing', field_name;
        END IF;

        -- Skip validation if field is not provided and not required
        IF field_value IS NULL AND NOT is_required THEN
            CONTINUE;
        END IF;

        -- Type validation
        CASE field_type
            WHEN 'string' THEN
                IF jsonb_typeof(field_value) != 'string' THEN
                    RAISE EXCEPTION 'Field % must be a string', field_name;
                END IF;
            WHEN 'number' THEN
                IF jsonb_typeof(field_value) != 'number' THEN
                    RAISE EXCEPTION 'Field % must be a number', field_name;
                END IF;
            WHEN 'boolean' THEN
                IF jsonb_typeof(field_value) != 'boolean' THEN
                    RAISE EXCEPTION 'Field % must be a boolean', field_name;
                END IF;
            WHEN 'date' THEN
                IF jsonb_typeof(field_value) != 'string' THEN
                    RAISE EXCEPTION 'Field % must be a date string', field_name;
                END IF;
                -- Additional date format validation here
        END CASE;
    END LOOP;

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;
```

## Data Migration Strategy

1. **Initial Migration**: Run all SQL migration files in order
2. **Schema Updates**: Use numbered migration files with version tracking
3. **Data Validation**: Validate data integrity after each migration
4. **Rollback Strategy**: Provide rollback scripts for each migration

## Performance Optimizations

1. **Indexing Strategy**:
   - Primary indexes on foreign keys
   - GIN indexes on JSONB columns
   - Full-text search indexes
   - Composite indexes for common queries

2. **Query Optimization**:
   - Use materialized views for complex aggregations
   - Implement pagination for large datasets
   - Use connection pooling
   - Cache frequently accessed data

3. **Storage Optimization**:
   - Use JSONB compression for large attribute objects
   - Archive old audit log entries
   - Partition large tables by date