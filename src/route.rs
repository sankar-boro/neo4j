use crate::{user, query};

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/get_user/{userid}", web::get().to(user::get_user));
  config.route("/create_user", web::post().to(user::create_user));
  config.route("/test_create_user", web::post().to(user::test_create_user));

  config.service(
    web::scope("/cquery")
    .route("/hit_query", web::post().to(query::hit_query))
    .route("/hit_queries", web::post().to(query::hit_queries))
    .route("/hit_get_queries", web::post().to(query::hit_get_queries))
  );

}