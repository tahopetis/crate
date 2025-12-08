-- Audit log table
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    old_values JSONB,
    new_values JSONB,
    performed_by UUID NOT NULL REFERENCES users(id),
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for audit log
CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_performed_by ON audit_log(performed_by);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at);
CREATE INDEX idx_audit_log_action ON audit_log(action);

-- CI Lifecycle Status table
CREATE TABLE ci_lifecycle_status (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ci_asset_id UUID NOT NULL REFERENCES ci_assets(id),
    status VARCHAR(50) NOT NULL,
    status_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    notes TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(ci_asset_id, status_date)
);

-- Indexes for lifecycle status
CREATE INDEX idx_ci_lifecycle_asset_id ON ci_lifecycle_status(ci_asset_id);
CREATE INDEX idx_ci_lifecycle_status ON ci_lifecycle_status(status);
CREATE INDEX idx_ci_lifecycle_date ON ci_lifecycle_status(status_date);

-- Trigger for updated_at
CREATE TRIGGER update_ci_lifecycle_status_updated_at BEFORE UPDATE ON ci_lifecycle_status
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();