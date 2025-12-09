-- Configurable Lifecycle Types table
CREATE TABLE lifecycle_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    default_color VARCHAR(7) DEFAULT '#6B7280', -- Hex color code
    is_active BOOLEAN DEFAULT true,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

-- Lifecycle States (the actual states within a lifecycle type)
CREATE TABLE lifecycle_states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lifecycle_type_id UUID NOT NULL REFERENCES lifecycle_types(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    color VARCHAR(7) DEFAULT '#6B7280', -- Hex color code for state
    order_index INTEGER NOT NULL,
    is_initial_state BOOLEAN DEFAULT false,
    is_terminal_state BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    UNIQUE(lifecycle_type_id, name),
    UNIQUE(lifecycle_type_id, order_index),
    CHECK (order_index >= 0),
    CHECK (color ~ '^#[0-9A-Fa-f]{6}$') -- Valid hex color
);

-- Lifecycle State Transitions (valid transitions between states)
CREATE TABLE lifecycle_transitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lifecycle_type_id UUID NOT NULL REFERENCES lifecycle_types(id) ON DELETE CASCADE,
    from_state_id UUID REFERENCES lifecycle_states(id) ON DELETE CASCADE,
    to_state_id UUID NOT NULL REFERENCES lifecycle_states(id) ON DELETE CASCADE,
    transition_name VARCHAR(100),
    description TEXT,
    requires_approval BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(lifecycle_type_id, from_state_id, to_state_id),
    CHECK (from_state_id != to_state_id)
);

-- CI Types to Lifecycle Types mapping
CREATE TABLE ci_type_lifecycles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ci_type_id UUID NOT NULL REFERENCES ci_types(id) ON DELETE CASCADE,
    lifecycle_type_id UUID NOT NULL REFERENCES lifecycle_types(id) ON DELETE CASCADE,
    is_default BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(ci_type_id, lifecycle_type_id)
);

-- Indexes for lifecycle_types
CREATE INDEX idx_lifecycle_types_name ON lifecycle_types(name);
CREATE INDEX idx_lifecycle_types_active ON lifecycle_types(is_active);
CREATE INDEX idx_lifecycle_types_created_by ON lifecycle_types(created_by);

-- Indexes for lifecycle_states
CREATE INDEX idx_lifecycle_states_type_id ON lifecycle_states(lifecycle_type_id);
CREATE INDEX idx_lifecycle_states_order ON lifecycle_states(lifecycle_type_id, order_index);
CREATE INDEX idx_lifecycle_states_initial ON lifecycle_states(is_initial_state);
CREATE INDEX idx_lifecycle_states_terminal ON lifecycle_states(is_terminal_state);

-- Indexes for lifecycle_transitions
CREATE INDEX idx_lifecycle_transitions_type_id ON lifecycle_transitions(lifecycle_type_id);
CREATE INDEX idx_lifecycle_transitions_from_state ON lifecycle_transitions(from_state_id);
CREATE INDEX idx_lifecycle_transitions_to_state ON lifecycle_transitions(to_state_id);

-- Indexes for ci_type_lifecycles
CREATE INDEX idx_ci_type_lifecycles_ci_type ON ci_type_lifecycles(ci_type_id);
CREATE INDEX idx_ci_type_lifecycles_lifecycle ON ci_type_lifecycles(lifecycle_type_id);
CREATE INDEX idx_ci_type_lifecycles_default ON ci_type_lifecycles(is_default);

-- Triggers for updated_at
CREATE TRIGGER update_lifecycle_types_updated_at BEFORE UPDATE ON lifecycle_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_lifecycle_states_updated_at BEFORE UPDATE ON lifecycle_states
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Audit trigger functions
CREATE OR REPLACE FUNCTION lifecycle_types_audit_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, new_values, performed_by)
        VALUES ('lifecycle_types', NEW.id, 'INSERT', row_to_json(NEW), NEW.created_by);
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, old_values, new_values, performed_by)
        VALUES ('lifecycle_types', NEW.id, 'UPDATE', row_to_json(OLD), row_to_json(NEW),
                COALESCE(NEW.created_by, OLD.created_by));
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, old_values, performed_by)
        VALUES ('lifecycle_types', OLD.id, 'DELETE', row_to_json(OLD), OLD.created_by);
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION lifecycle_states_audit_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, new_values, performed_by)
        VALUES ('lifecycle_states', NEW.id, 'INSERT', row_to_json(NEW), COALESCE(NEW.created_by, (SELECT created_by FROM users LIMIT 1)));
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, old_values, new_values, performed_by)
        VALUES ('lifecycle_states', NEW.id, 'UPDATE', row_to_json(OLD), row_to_json(NEW),
                COALESCE(NEW.created_by, (SELECT created_by FROM users LIMIT 1)));
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO audit_log (entity_type, entity_id, action, old_values, performed_by)
        VALUES ('lifecycle_states', OLD.id, 'DELETE', row_to_json(OLD), COALESCE(OLD.created_by, (SELECT created_by FROM users LIMIT 1)));
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Apply audit triggers
CREATE TRIGGER lifecycle_types_audit
    AFTER INSERT OR UPDATE OR DELETE ON lifecycle_types
    FOR EACH ROW EXECUTE FUNCTION lifecycle_types_audit_trigger();

CREATE TRIGGER lifecycle_states_audit
    AFTER INSERT OR UPDATE OR DELETE ON lifecycle_states
    FOR EACH ROW EXECUTE FUNCTION lifecycle_states_audit_trigger();