use axum::http::HeaderMap;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;


pub async fn single_extractor(TypedHeader(agent): TypedHeader<UserAgent>) -> String {
    format!("Hello there, Your agent: {}", agent.as_str())
}

pub async fn mapped_extractor(headers: HeaderMap) {
    for (key, value) in headers.iter() {
        println!("Header: {}\tValue: {:?}", key, value);
    }
}
