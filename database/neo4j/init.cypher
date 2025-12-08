-- Neo4j Initialization Script
-- This script sets up the graph database constraints and indexes

-- Show current user and database
SHOW CURRENT USER;
SHOW CURRENT DATABASE;

-- Create constraints for uniqueness
CREATE CONSTRAINT ci_id_unique IF NOT EXISTS FOR (c:CI) REQUIRE c.id IS UNIQUE;
CREATE CONSTRAINT ci_type_name_unique IF NOT EXISTS FOR (t:CIType) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT user_id_unique IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE;
CREATE CONSTRAINT rel_type_name_unique IF NOT EXISTS FOR (r:RelType) REQUIRE r.name IS UNIQUE;

-- Create indexes for performance
CREATE INDEX ci_name_index IF NOT EXISTS FOR (c:CI) ON (c.name);
CREATE INDEX ci_type_index IF NOT EXISTS FOR (c:CI) ON (c.ci_type_name);
CREATE INDEX ci_lifecycle_index IF NOT EXISTS FOR (c:CI) ON (c.lifecycle_name);
CREATE INDEX ci_created_at_index IF NOT EXISTS FOR (c:CI) ON (c.created_at);
CREATE INDEX ci_updated_at_index IF NOT EXISTS FOR (c:CI) ON (c.updated_at);

-- Create full-text search index
CREATE FULLTEXT INDEX ci_search_index IF NOT EXISTS FOR (c:CI) ON EACH [c.name, c.description];

-- Create sample data for testing
MERGE (admin:User {id: '00000000-0000-0000-0000-000000000001',
                    name: 'System Admin',
                    email: 'admin@crate.local',
                    created_at: datetime()});

-- Create sample CI types
MERGE (server_type:CIType {id: 'server-type-uuid',
                           name: 'Server',
                           description: 'Physical or virtual server',
                           icon: 'server',
                           color: '#3b82f6',
                           created_at: datetime()});

MERGE (database_type:CIType {id: 'db-type-uuid',
                             name: 'Database',
                             description: 'Database instance or cluster',
                             icon: 'database',
                             color: '#10b981',
                             created_at: datetime()});

MERGE (app_type:CIType {id: 'app-type-uuid',
                        name: 'Application',
                        description: 'Software application or service',
                        icon: 'package',
                        color: '#f59e0b',
                        created_at: datetime()});

-- Create relationship types
MERGE (depends_rel:RelType {name: 'DEPENDS_ON',
                            description: 'CI depends on another CI',
                            color: '#ef4444',
                            is_bidirectional: false,
                            created_at: datetime()});

MERGE (hosts_rel:RelType {name: 'HOSTS',
                          description: 'CI hosts another CI',
                          color: '#8b5cf6',
                          is_bidirectional: false,
                          created_at: datetime()});

MERGE (connects_rel:RelType {name: 'CONNECTS_TO',
                             description: 'CI connects to another CI',
                             color: '#06b6d4',
                             is_bidirectional: true,
                             created_at: datetime()});

-- Print setup completion
RETURN 'Neo4j initialization completed successfully' AS message;