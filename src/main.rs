use axum::{ routing::{get, post}, Router, response::IntoResponse, http::StatusCode };

#[tokio::main]
async fn main() {
    let people = HashMap::new();

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Returned")
}

async fn find_person() -> impl IntoResponse {
    StatusCode::OK
}

async fn create_person() -> impl IntoResponse {
    StatusCode::OK
}

async fn count_people() -> impl IntoResponse {
    StatusCode::OK
}
