use std::sync::Arc;
use neo4rs::{Graph, query, Node};

#[actix_web::main]
async fn main() {
  let uri = "neo4j:sankarboro@127.0.0.1:7687";
    let user = "neo4j";
    let pass = "sankarboro";
    let graph = Arc::new(Graph::new(uri.to_string(), user.to_string(), pass.to_string()).await.unwrap());
    for _ in 1..=42 {
        let graph = graph.clone();
        tokio::spawn(async move {
            let mut result = graph.execute(
	       query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Sankar boro")
	    ).await.unwrap();
            while let Ok(Some(row)) = result.next().await {
        	let node: Node = row.get("p").unwrap();
        	let name: String = node.get("name").unwrap();
                println!("{}", name);
            }
        });
    }
}