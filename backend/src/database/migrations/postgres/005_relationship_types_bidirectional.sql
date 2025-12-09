-- Add bidirectional support to relationship_types table
ALTER TABLE relationship_types
ADD COLUMN is_bidirectional BOOLEAN DEFAULT false,
ADD COLUMN reverse_name VARCHAR(255);

-- Add comments
COMMENT ON COLUMN relationship_types.is_bidirectional IS 'Whether this relationship type is bidirectional';
COMMENT ON COLUMN relationship_types.reverse_name IS 'The name for the reverse relationship (used when is_bidirectional is true)';

-- Add indexes for performance
CREATE INDEX idx_relationship_types_bidirectional ON relationship_types(is_bidirectional);
CREATE INDEX idx_relationship_types_from_ci_type_id ON relationship_types(from_ci_type_id);
CREATE INDEX idx_relationship_types_to_ci_type_id ON relationship_types(to_ci_type_id);

-- Trigger already created in 001_initial_schema.sql

-- Add constraint to ensure reverse_name is provided when bidirectional
ALTER TABLE relationship_types
ADD CONSTRAINT check_reverse_name_required
CHECK (
    NOT is_bidirectional OR (reverse_name IS NOT NULL AND reverse_name != '')
);

-- Add constraint to prevent self-relationships
ALTER TABLE relationship_types
ADD CONSTRAINT check_no_self_relationship
CHECK (
    from_ci_type_id IS NULL OR to_ci_type_id IS NULL OR from_ci_type_id <> to_ci_type_id
);