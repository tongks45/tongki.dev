use axum::{Router, routing::get, Extension, response::{Html, Response, IntoResponse}, middleware::{from_fn}, http::{StatusCode}, Json};
use serde::{Serialize};
use serde_json::json;
use askama::Template;


enum ApiError{

}
#[derive(Serialize)]
struct Message {
    message: String
}
enum ApiOK {
    Ok,
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

impl IntoResponse for ApiOK {
    fn into_response(self) -> Response {
        let template = IndexTemplate {
            name: "Tongki Ilagan"
        };
        match self {
            Self::Ok => (StatusCode::OK, Html(template.render().unwrap())).into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    header: &'a str,
    message: &'a str,
}

use crate::middlewares::request_logger::request_logger;
#[derive(Serialize)]
struct not_found_msg {
    header: String,
    message: String
}
pub fn routes() -> Router {
    Router::new()
        .route("/", get(root).route_layer(from_fn(request_logger)))
        .fallback(not_found)
}
async fn root() -> ApiOK{

    //let html = state.h.render("index", &json!({})).unwrap();
    ApiOK::Ok
}
async fn not_found() -> impl IntoResponse {
    let template = NotFoundTemplate {
        header: "404",
        message: "Page cannot be Found"
    };
    (StatusCode::OK, Html(template.render().unwrap()).into_response())
}