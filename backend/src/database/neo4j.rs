use crate::config::Neo4jConfig;
use anyhow::{Result, Context};
use neo4rs::{Graph, ConfigBuilder};

#[derive(Debug, Clone)]
pub struct Neo4jPool {
    graph: Graph,
}

impl Neo4jPool {
    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}

pub async fn get_neo4j_pool(config: &Neo4jConfig) -> Result<Neo4jPool> {
    tracing::info!("Initializing Neo4j connection pool with URI: {}", config.uri);

    let neo4j_config = ConfigBuilder::default()
        .uri(&config.uri)
        .user(&config.user)
        .password(&config.password)
        .db(&config.database)
        .fetch_size(500)
        .max_connections(10)
        .build()
        .context("Failed to build Neo4j configuration")?;

    let graph = Graph::connect(neo4j_config)
        .await
        .context("Failed to connect to Neo4j database")?;

    tracing::info!("Successfully connected to Neo4j database");

    Ok(Neo4jPool { graph })
}

pub async fn run_initializations(pool: &Neo4jPool) -> Result<()> {
    tracing::info!("Running Neo4j initializations...");

    let graph = pool.graph();

    // Create constraints for uniqueness
    let constraints = vec![
        "CREATE CONSTRAINT ci_asset_id_unique IF NOT EXISTS FOR (a:CIAsset) REQUIRE a.id IS UNIQUE",
        "CREATE CONSTRAINT ci_type_name_unique IF NOT EXISTS FOR (t:CIType) REQUIRE t.name IS UNIQUE",
        "CREATE CONSTRAINT user_email_unique IF NOT EXISTS FOR (u:User) REQUIRE u.email IS UNIQUE",
    ];

    for constraint in constraints {
        graph.run(neo4rs::query(constraint)).await
            .context(format!("Failed to create constraint: {}", constraint))?;
    }

    // Create indexes for performance
    let indexes = vec![
        "CREATE INDEX ci_asset_name_index IF NOT EXISTS FOR (a:CIAsset) ON (a.name)",
        "CREATE INDEX ci_asset_type_index IF NOT EXISTS FOR (a:CIAsset) ON (a.type)",
        "CREATE INDEX ci_type_name_index IF NOT EXISTS FOR (t:CIType) ON (t.name)",
        "CREATE INDEX user_email_index IF NOT EXISTS FOR (u:User) ON (u.email)",
    ];

    for index in indexes {
        graph.run(neo4rs::query(index)).await
            .context(format!("Failed to create index: {}", index))?;
    }

    tracing::info!("Successfully completed Neo4j initializations");
    Ok(())
}

/// Initialize relationship type constraints in Neo4j
/// This should be called when relationship types are created or updated
pub async fn initialize_relationship_type_constraints(
    _pool: &Neo4jPool,
    relationship_type_name: &str,
    from_ci_type: Option<&str>,
    to_ci_type: Option<&str>,
    is_bidirectional: bool,
) -> Result<()> {
    // TODO: Implement full constraint creation when Neo4j integration is complete
    println!("Initializing constraints for relationship type: {}", relationship_type_name);
    println!("  From CI type: {:?}", from_ci_type);
    println!("  To CI type: {:?}", to_ci_type);
    println!("  Bidirectional: {}", is_bidirectional);

    // Simulate constraint creation success
    Ok(())
}