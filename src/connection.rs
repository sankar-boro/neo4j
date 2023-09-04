use std::sync::Arc;
use neo4rs::Graph;

pub async fn get_neo4j_connection() -> Arc<Graph> {
  // concurrent queries
     let uri = "neo4j:sankarboro@127.0.0.1:7687";
     let user = "neo4j";
     let pass = "sankarboro";
     let graph = Arc::new(Graph::new(uri.to_string(), user.to_string(), pass.to_string()).await.unwrap());
     graph
}