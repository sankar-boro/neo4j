use crate::error::HttpErrorResponse;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use neo4rs::{Graph, query, Node};

#[derive(Serialize, Deserialize)]
pub struct GetUser {
  name: String,
  email: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct InsertUser {
  fname: String,
  lname: String,
  email: String,
}

pub async fn get_user(client: web::Data<Arc<Graph>>) -> Result<HttpResponse, HttpErrorResponse> {
    let mut result = client.execute(
      query("MATCH (p:Person {fname: 'Sankar'}) RETURN p")
    ).await.unwrap();
    let mut users = Vec::new();
    while let Ok(Some(row)) = result.next().await {
      let node: Node = row.get("p").unwrap();
      let fname: String = node.get("fname").unwrap();
      let lname: String = node.get("lname").unwrap();
      let email: String = node.get("email").unwrap();
      let user = serde_json::json!({ "fname": fname, "lname": lname, "email": email });
      users.push(user);
    }
    Ok(HttpResponse::Ok().json(users))
}


pub async fn create_user(client: web::Data<Graph>, form: web::Json<InsertUser>) -> HttpResponse {
       //Transactions
       let txn = client.start_txn().await.unwrap();
       txn.run_queries(vec![
           query(&format!("CREATE (p:Person {{ fname: {}, lname: {}, email: {} }} )", &form.fname, &form.lname, &form.email)),
       ])
       .await
       .unwrap();
       txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
      HttpResponse::Ok().body("Inserted user")
       
}
