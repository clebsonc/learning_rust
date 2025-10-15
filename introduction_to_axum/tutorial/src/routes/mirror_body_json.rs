use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
    price: f64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonResponse {
    message: String,
    price: f64
}


pub async fn mirror_body_json(Json(payload): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    println!("bbbbbb");
    let c = MirrorJsonResponse{
        message: String::from("You own me."),
        price: payload.price * 2.0
    };

    Json(c)
}
