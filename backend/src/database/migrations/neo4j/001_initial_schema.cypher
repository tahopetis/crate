// Create constraints for uniqueness
CREATE CONSTRAINT ci_asset_id_unique IF NOT EXISTS FOR (a:CIAsset) REQUIRE a.id IS UNIQUE;
CREATE CONSTRAINT ci_type_name_unique IF NOT EXISTS FOR (t:CIType) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT user_email_unique IF NOT EXISTS FOR (u:User) REQUIRE u.email IS UNIQUE;

// Create indexes for performance
CREATE INDEX ci_asset_name_index IF NOT EXISTS FOR (a:CIAsset) ON (a.name);
CREATE INDEX ci_asset_type_index IF NOT EXISTS FOR (a:CIAsset) ON (a.type);
CREATE INDEX ci_type_name_index IF NOT EXISTS FOR (t:CIType) ON (t.name);
CREATE INDEX user_email_index IF NOT EXISTS FOR (u:User) ON (u.email);

// Create sample relationship types
// These will be managed dynamically through the application
// Example relationships that might be used:
// - DEPENDS_ON
// - CONTAINS
// - MANAGED_BY
// - HOSTED_ON
// - CONNECTS_TO
// - VERSION_OF