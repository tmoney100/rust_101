use axum::{
  Router,
  http::{
    // header::{AUTHORIZATION, CONTENT_TYPE},
    request::Parts,
    HeaderValue,
    Method,
  }
};

fn main () {
  let app = Router::new()
    .route("/")
      .route("/api/1")
        .nest("/person", routes::get_forecast2::mount());

}