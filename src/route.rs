use crate::user;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/get_user", web::get().to(user::get_user));
  config.route("/create_user", web::post().to(user::create_user));

  // config.service(
  //   web::scope("/user")
  //   .route("/session", web::get().to(user::user_session))
  //   .route("/get/{user_id}", web::get().to(user::get_user))
  //   .route("/update/{user_id}", web::post().to(user::update_user))
  //   .route("/delete/{user_id}", web::post().to(user::delete_user))
  // );

}