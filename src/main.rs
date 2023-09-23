use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use time::macros::date;
use time::Date;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birthdate: Date,
    pub stack: Vec<String>,
}

type AppState = Arc<HashMap<Uuid, Person>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Vitor Trimer"),
        nick: String::from("VTR"),
        birthdate: date!(1995 - 06 - 10),
        stack: vec!["Swift".to_string(), "Rust".to_string()],
    };

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(people);

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people(state: State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Returned")
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    match people.get(&person_id) {
        Some(person) => Ok(Json(person)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person() -> impl IntoResponse {
    StatusCode::OK
}

async fn count_people() -> impl IntoResponse {
    StatusCode::OK
}
