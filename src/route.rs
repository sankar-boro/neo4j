use crate::user;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/get_user/{userid}", web::get().to(user::get_user));
  config.route("/create_user", web::post().to(user::create_user));
}