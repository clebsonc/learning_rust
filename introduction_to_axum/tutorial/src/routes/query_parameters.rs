use axum::extract::Query;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorQuery {
    pub message: String,

    #[serde(alias="identificator")]
    pub id: String,

    pub value: Option<f64>
}



pub async fn query_parameters(Query(params): Query<MirrorQuery>) -> String {
    let value = params.value.unwrap_or(10.0);
    format!( "Received message: {} -- Received ID: {} \n value: {}", params.message ,  params.id, value)
}
