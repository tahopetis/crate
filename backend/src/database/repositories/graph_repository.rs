use crate::database::Neo4jPool;
use anyhow::{Result, Context};
use serde_json::Value;
use uuid::Uuid;
use neo4rs::{query, BoltType};

#[derive(Debug)]
pub struct GraphRepository {
    pool: Neo4jPool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphNode {
    pub id: Uuid,
    pub name: String,
    pub ci_type: String,
    pub ci_type_id: Uuid,
    pub attributes: Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphRelationship {
    pub id: Option<Uuid>,
    pub relationship_type: String,
    pub from_node_id: Uuid,
    pub to_node_id: Uuid,
    pub attributes: Value,
    pub from_ci_type: String,
    pub to_ci_type: String,
}

impl GraphRepository {
    pub fn new(pool: Neo4jPool) -> Self {
        Self { pool }
    }

    /// Create or update a CI asset node in Neo4j
    pub async fn create_ci_node(
        &self,
        asset_id: Uuid,
        name: &str,
        ci_type: &str,
        ci_type_id: Uuid,
        attributes: &Value,
    ) -> Result<()> {
        let graph = self.pool.graph();

        let cypher = r#"
            MERGE (a:CIAsset {id: $asset_id})
            SET a.name = $name,
                a.type = $ci_type,
                a.type_id = $ci_type_id,
                a.attributes = $attributes,
                a.updated_at = datetime()
            RETURN a
        "#;

        let q = query(cypher)
            .param("asset_id", asset_id.to_string())
            .param("name", name)
            .param("ci_type", ci_type)
            .param("ci_type_id", ci_type_id.to_string())
            .param("attributes", serde_json::to_string(attributes)?);

        graph.run(q).await
            .context("Failed to create/update CI node in Neo4j")?;

        tracing::debug!("Created/updated CI node in Neo4j: {} ({})", name, asset_id);
        Ok(())
    }

    /// Create or update a CI type node in Neo4j
    pub async fn create_ci_type_node(
        &self,
        type_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<()> {
        let graph = self.pool.graph();

        let cypher = r#"
            MERGE (t:CIType {name: $name})
            SET t.id = $type_id,
                t.description = $description,
                t.updated_at = datetime()
            RETURN t
        "#;

        let q = query(cypher)
            .param("type_id", type_id.to_string())
            .param("name", name)
            .param("description", description.unwrap_or(""));

        graph.run(q).await
            .context("Failed to create/update CI type node in Neo4j")?;

        tracing::debug!("Created/updated CI type node in Neo4j: {} ({})", name, type_id);
        Ok(())
    }

    /// Create a relationship between two CI assets
    pub async fn create_relationship(
        &self,
        from_asset_id: Uuid,
        to_asset_id: Uuid,
        relationship_type: &str,
        relationship_type_id: Uuid,
        properties: Option<Value>,
        from_ci_type: &str,
        to_ci_type: &str,
        is_bidirectional: bool,
    ) -> Result<()> {
        let graph = self.pool.graph();

        // Sanitize relationship type for Neo4j (replace spaces/special chars with underscores)
        let rel_type_name = relationship_type
            .to_uppercase()
            .replace(" ", "_")
            .replace("-", "_");

        let props = properties.unwrap_or(Value::Object(serde_json::Map::new()));

        let cypher = format!(r#"
            MATCH (from:CIAsset {{id: $from_id}})
            MATCH (to:CIAsset {{id: $to_id}})
            MERGE (from)-[r:{} {{type_id: $type_id}}]->(to)
            SET r.attributes = $attributes,
                r.from_ci_type = $from_ci_type,
                r.to_ci_type = $to_ci_type,
                r.is_bidirectional = $is_bidirectional,
                r.created_at = coalesce(r.created_at, datetime()),
                r.updated_at = datetime()
            RETURN r
        "#, rel_type_name);

        let q = query(&cypher)
            .param("from_id", from_asset_id.to_string())
            .param("to_id", to_asset_id.to_string())
            .param("type_id", relationship_type_id.to_string())
            .param("attributes", serde_json::to_string(&props)?)
            .param("from_ci_type", from_ci_type)
            .param("to_ci_type", to_ci_type)
            .param("is_bidirectional", is_bidirectional);

        graph.run(q).await
            .context("Failed to create relationship in Neo4j")?;

        tracing::debug!(
            "Created relationship in Neo4j: {} -> {} ({})",
            from_asset_id,
            to_asset_id,
            relationship_type
        );

        Ok(())
    }

    /// Delete a CI asset node and all its relationships
    pub async fn delete_node(&self, asset_id: Uuid) -> Result<()> {
        let graph = self.pool.graph();

        let cypher = r#"
            MATCH (a:CIAsset {id: $asset_id})
            DETACH DELETE a
        "#;

        let q = query(cypher)
            .param("asset_id", asset_id.to_string());

        graph.run(q).await
            .context("Failed to delete CI node from Neo4j")?;

        tracing::debug!("Deleted CI node from Neo4j: {}", asset_id);
        Ok(())
    }

    /// Delete a specific relationship
    pub async fn delete_relationship(
        &self,
        from_asset_id: Uuid,
        to_asset_id: Uuid,
        relationship_type_id: Uuid,
    ) -> Result<()> {
        let graph = self.pool.graph();

        let cypher = r#"
            MATCH (from:CIAsset {id: $from_id})-[r {type_id: $type_id}]->(to:CIAsset {id: $to_id})
            DELETE r
        "#;

        let q = query(cypher)
            .param("from_id", from_asset_id.to_string())
            .param("to_id", to_asset_id.to_string())
            .param("type_id", relationship_type_id.to_string());

        graph.run(q).await
            .context("Failed to delete relationship from Neo4j")?;

        tracing::debug!("Deleted relationship from Neo4j: {} -> {}", from_asset_id, to_asset_id);
        Ok(())
    }

    /// Get all nodes related to a specific CI asset
    pub async fn get_related_nodes(
        &self,
        asset_id: Uuid,
    ) -> Result<Vec<(Uuid, String, String, String, Value)>> {
        let graph = self.pool.graph();

        let cypher = r#"
            MATCH (a:CIAsset {id: $asset_id})-[r]-(related:CIAsset)
            RETURN DISTINCT related.id as id, related.name as name,
                   related.type as ci_type, type(r) as rel_type,
                   related.attributes as attributes
            LIMIT 100
        "#;

        let q = query(cypher)
            .param("asset_id", asset_id.to_string());

        let mut result = graph.execute(q).await
            .context("Failed to get related nodes from Neo4j")?;

        let mut related_nodes = Vec::new();

        while let Some(row) = result.next().await? {
            let id_str: String = row.get("id").unwrap_or_default();
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let name: String = row.get("name").unwrap_or_default();
            let ci_type: String = row.get("ci_type").unwrap_or_default();
            let rel_type: String = row.get("rel_type").unwrap_or_default();
            let attrs_str: String = row.get("attributes").unwrap_or_else(|_| "{}".to_string());
            let attributes: Value = serde_json::from_str(&attrs_str).unwrap_or(Value::Object(serde_json::Map::new()));

            related_nodes.push((id, name, ci_type, rel_type, attributes));
        }

        Ok(related_nodes)
    }

    /// Get the full graph with optional filtering
    pub async fn get_full_graph(
        &self,
        node_limit: Option<u32>,
        ci_type_filter: Option<&str>,
    ) -> Result<(Vec<GraphNode>, Vec<GraphRelationship>)> {
        let graph = self.pool.graph();

        let limit = node_limit.unwrap_or(1000);

        // Build the MATCH clause with optional type filter
        let match_clause = if let Some(ci_type) = ci_type_filter {
            format!("MATCH (a:CIAsset {{type: $ci_type}})")
        } else {
            "MATCH (a:CIAsset)".to_string()
        };

        let node_cypher = format!(r#"
            {}
            RETURN a.id as id, a.name as name, a.type as ci_type,
                   a.type_id as ci_type_id, a.attributes as attributes
            LIMIT $limit
        "#, match_clause);

        let mut q = query(&node_cypher)
            .param("limit", limit as i64);

        if let Some(ci_type) = ci_type_filter {
            q = q.param("ci_type", ci_type);
        }

        let mut result = graph.execute(q).await
            .context("Failed to get nodes from Neo4j")?;

        let mut nodes = Vec::new();
        let mut node_ids = Vec::new();

        while let Some(row) = result.next().await? {
            let id_str: String = row.get("id").unwrap_or_default();
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let name: String = row.get("name").unwrap_or_default();
            let ci_type: String = row.get("ci_type").unwrap_or_default();
            let type_id_str: String = row.get("ci_type_id").unwrap_or_default();
            let ci_type_id = Uuid::parse_str(&type_id_str).unwrap_or_default();
            let attrs_str: String = row.get("attributes").unwrap_or_else(|_| "{}".to_string());
            let attributes: Value = serde_json::from_str(&attrs_str).unwrap_or(Value::Object(serde_json::Map::new()));

            node_ids.push(id_str.clone());
            nodes.push(GraphNode {
                id,
                name,
                ci_type,
                ci_type_id,
                attributes,
            });
        }

        // Now get relationships between these nodes
        let mut relationships = Vec::new();

        if !node_ids.is_empty() {
            let rel_cypher = r#"
                MATCH (from:CIAsset)-[r]->(to:CIAsset)
                WHERE from.id IN $node_ids AND to.id IN $node_ids
                RETURN type(r) as rel_type, from.id as from_id, to.id as to_id,
                       from.type as from_type, to.type as to_type,
                       r.attributes as attributes
            "#;

            let q = query(rel_cypher)
                .param("node_ids", node_ids);

            let mut result = graph.execute(q).await
                .context("Failed to get relationships from Neo4j")?;

            while let Some(row) = result.next().await? {
                let rel_type: String = row.get("rel_type").unwrap_or_default();
                let from_id_str: String = row.get("from_id").unwrap_or_default();
                let to_id_str: String = row.get("to_id").unwrap_or_default();
                let from_id = Uuid::parse_str(&from_id_str).unwrap_or_default();
                let to_id = Uuid::parse_str(&to_id_str).unwrap_or_default();
                let from_type: String = row.get("from_type").unwrap_or_default();
                let to_type: String = row.get("to_type").unwrap_or_default();
                let attrs_str: String = row.get("attributes").unwrap_or_else(|_| "{}".to_string());
                let attributes: Value = serde_json::from_str(&attrs_str).unwrap_or(Value::Object(serde_json::Map::new()));

                relationships.push(GraphRelationship {
                    id: None,
                    relationship_type: rel_type,
                    from_node_id: from_id,
                    to_node_id: to_id,
                    attributes,
                    from_ci_type: from_type,
                    to_ci_type: to_type,
                });
            }
        }

        Ok((nodes, relationships))
    }

    /// Search for CI assets using full-text search
    pub async fn search_assets(&self, search_term: &str, limit: Option<u32>) -> Result<Vec<GraphNode>> {
        let graph = self.pool.graph();

        let search_limit = limit.unwrap_or(20);

        let cypher = r#"
            MATCH (a:CIAsset)
            WHERE toLower(a.name) CONTAINS toLower($search_term)
               OR toLower(a.type) CONTAINS toLower($search_term)
            RETURN a.id as id, a.name as name, a.type as ci_type,
                   a.type_id as ci_type_id, a.attributes as attributes
            LIMIT $limit
        "#;

        let q = query(cypher)
            .param("search_term", search_term)
            .param("limit", search_limit as i64);

        let mut result = graph.execute(q).await
            .context("Failed to search assets in Neo4j")?;

        let mut nodes = Vec::new();

        while let Some(row) = result.next().await? {
            let id_str: String = row.get("id").unwrap_or_default();
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let name: String = row.get("name").unwrap_or_default();
            let ci_type: String = row.get("ci_type").unwrap_or_default();
            let type_id_str: String = row.get("ci_type_id").unwrap_or_default();
            let ci_type_id = Uuid::parse_str(&type_id_str).unwrap_or_default();
            let attrs_str: String = row.get("attributes").unwrap_or_else(|_| "{}".to_string());
            let attributes: Value = serde_json::from_str(&attrs_str).unwrap_or(Value::Object(serde_json::Map::new()));

            nodes.push(GraphNode {
                id,
                name,
                ci_type,
                ci_type_id,
                attributes,
            });
        }

        Ok(nodes)
    }

    /// Initialize relationship type constraints for a new relationship type
    pub async fn initialize_relationship_constraints(
        &self,
        relationship_type: &str,
        _from_ci_type: Option<&str>,
        _to_ci_type: Option<&str>,
        _is_bidirectional: bool,
    ) -> Result<()> {
        // Neo4j doesn't require pre-creating relationship types
        // They are created dynamically when relationships are created
        tracing::debug!("Relationship type registered in Neo4j: {}", relationship_type);
        Ok(())
    }
}
