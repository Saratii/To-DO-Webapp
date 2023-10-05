use std::{sync::Arc, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use axum::{
    extract::{State, Json},
    routing::{get, post, delete},
    Router, http::{Method, HeaderName}
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use tokio::sync::Mutex;

mod db_interact;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    // pub creation_date: String,
    // pub due_date: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AppState{
    pub database_connection: Pool<Postgres>
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(vec![HeaderName::from_static("content-type")]);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:mommy@localhost:5433/To_Do_Database")
        .await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    let shared_state = Arc::new(Mutex::new(AppState{database_connection: pool}));
    let app = Router::new()
        .route("/", get(root))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks", post(create_task).get(read_tasks))
        .layer(cors)
        .with_state(shared_state);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!, this is the root"
}

#[axum::debug_handler]
async fn create_task(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(task): Json<Task>,
) -> Json<Vec<Task>> {
    let state = state.lock().await;
    db_interact::insert_task(task, &state.database_connection).await;
    return Json(db_interact::read_tasks(&state.database_connection).await)
}

async fn read_tasks(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<Task>> {
    let state = state.lock().await;
    let tasks = db_interact::read_tasks(&state.database_connection);
    return Json(tasks.await);
}

async fn delete_task(
    State(state): State<Arc<Mutex<AppState>>>,
    axum::extract::Path(id): axum::extract::Path<u32>
) -> Json<Vec<Task>> {
    let state = state.lock().await;
    db_interact::delete_task(id, &state.database_connection).await;
    return Json(db_interact::read_tasks(&state.database_connection).await)
}