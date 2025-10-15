mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_parameters;
mod header_extractor;

use axum::{routing::{get, post}, Router};

use crate::routes::mirror_body_string::mirror_body_string;
use crate::routes::mirror_body_json::mirror_body_json;
use crate::routes::path_variables::path_variables;
use crate::routes::query_parameters::query_parameters;
use crate::routes::header_extractor::{single_extractor, mapped_extractor};


pub fn create_routes() -> Router {
    let app: Router = Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/{id}", post(path_variables))
        .route("/query_parameters", get(query_parameters))
        .route("/single_head", get(single_extractor))
        .route("/mapped_head", get(mapped_extractor));
    app
}



async fn hello_world() -> String {
    String::from("Hello World from my own file!")
}
