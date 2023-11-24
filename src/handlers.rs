use std::sync::Arc;

use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};

use crate::repositories::{TodoRepository, CreateTodo};


pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

