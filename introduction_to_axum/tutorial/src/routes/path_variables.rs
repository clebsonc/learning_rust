use axum::extract::Path;


pub async fn path_variables(Path(id): Path<i32>) -> String {
    format!("hi there {}", id)
}
