use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub serial: String,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: String,
    pub due_date: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:mommy@localhost:5433/To_Do_Database")
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT 150")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    let shared_state: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", post(create_task).get(read_tasks))
        .with_state(shared_state);
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!, this is the root"
}
async fn create_task(
    // Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<Mutex<Vec<Task>>>>,
) -> Json<Vec<Task>> {
    let task = Task {
        // serial: params.get("title").unwrap().to_string(),
        // title: params.get("title").unwrap().to_string(),
        // description: params.get("title").map(|d| d.to_string()),
        // creation_date: params.get("title").unwrap().to_string(),
        // due_date: params.get("title").map(|d| d.to_string()),
        serial: "hello world".to_owned(),
        title: "hello world".to_owned(),
        description: Some("hello world".to_owned()),
        creation_date: "hello world".to_owned(),
        due_date: Some("hello world".to_owned()),
    };
    let mut state = state.lock().await;
    state.push(task);
    let mut tasks: Vec<Task> = Vec::new();
    for task in state.iter() {
        tasks.push(task.clone());
    }
    return Json(tasks);
}

async fn read_tasks(State(state): State<Arc<Mutex<Vec<Task>>>>) -> Json<Vec<Task>> {
    let state = state.lock().await;
    let mut tasks: Vec<Task> = Vec::new();
    for task in state.iter() {
        tasks.push(task.clone());
    }
    return Json(tasks);
}
