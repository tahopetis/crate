-- Relationships table for storing actual relationship instances between CI assets
CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    relationship_type_id UUID NOT NULL REFERENCES relationship_types(id),
    from_ci_asset_id UUID NOT NULL REFERENCES ci_assets(id) ON DELETE CASCADE,
    to_ci_asset_id UUID NOT NULL REFERENCES ci_assets(id) ON DELETE CASCADE,
    attributes JSONB DEFAULT '{}',
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL,
    deleted_by UUID REFERENCES users(id),

    -- Ensure we don't create duplicate relationships
    CONSTRAINT unique_relationship UNIQUE (relationship_type_id, from_ci_asset_id, to_ci_asset_id, deleted_at)
);

-- Indexes for performance
CREATE INDEX idx_relationships_type_id ON relationships(relationship_type_id);
CREATE INDEX idx_relationships_from_asset ON relationships(from_ci_asset_id);
CREATE INDEX idx_relationships_to_asset ON relationships(to_ci_asset_id);
CREATE INDEX idx_relationships_created_by ON relationships(created_by);
CREATE INDEX idx_relationships_deleted_at ON relationships(deleted_at);

-- Composite index for finding all relationships for a given asset
CREATE INDEX idx_relationships_asset_lookup ON relationships(from_ci_asset_id, to_ci_asset_id)
    WHERE deleted_at IS NULL;

-- Trigger for updated_at
CREATE TRIGGER update_relationships_updated_at BEFORE UPDATE ON relationships
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Prevent self-referencing relationships
ALTER TABLE relationships
    ADD CONSTRAINT no_self_reference
    CHECK (from_ci_asset_id != to_ci_asset_id);
