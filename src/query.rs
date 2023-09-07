use crate::error::HttpErrorResponse;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::sync::Arc;
use neo4rs::{Graph, query, Node};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct HitQuery {
  inner: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct HitQueries {
  inner: Vec<String>,
}

pub async fn hit_query(client: web::Data<Arc<Graph>>, payload: web::Json<HitQuery>) -> Result<HttpResponse, HttpErrorResponse> {
    //Transactions
    let txn = client.start_txn().await?;
    let test_query = &payload.inner;
    txn.run_queries(vec![
        query(test_query.as_str().into()),
    ])
    .await
    .unwrap();
    txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
    Ok(HttpResponse::Ok().json(json!({ "status": 200, "data": "hit_query"}))) 
}
  
pub async fn hit_queries(client: web::Data<Arc<Graph>>, payload: web::Json<HitQueries>) -> Result<HttpResponse, HttpErrorResponse> {
    //Transactions
    let txn = client.start_txn().await.unwrap();
    let mut test_queries = Vec::new();
    let mut x = payload.inner.iter();
    while let Some(data) = x.next() {
        test_queries.push(query(data.as_str().into()));
    }
    txn.run_queries(test_queries)
    .await
    .unwrap();
    txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
    Ok(HttpResponse::Ok().json(json!({ "status": 200, "data": "hit_queries"})))
}

pub async fn hit_get_queries(client: web::Data<Arc<Graph>>, payload: web::Json<HitQuery>) -> Result<HttpResponse, HttpErrorResponse> {
    let query_ = query(payload.inner.as_str().into());
    let mut result = client.execute(
        query_
    ).await.unwrap();
    // let mut users = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("node").unwrap();
        let labels = node.labels();
        println!("labels: {:#?}", labels);
    }
    Ok(HttpResponse::Ok().body("hello"))
}