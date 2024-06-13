mod api;
mod qabase;
mod qna;

use api::*;
use qabase::*;
use qna::*;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Result},
    routing::{delete, get, post, put},
    Json, Router,
};

use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

use serde::{Deserialize, Serialize};
extern crate serde_json;
extern crate thiserror;
use tokio::{self, sync::RwLock};
extern crate tracing;
use utoipa::{OpenApi, ToSchema};

#[tokio::main]
async fn main() {
    let user = "postgres";
    let password = "postgres";
    let host = "localhost";
    let url = format!(
        "postgres://{}:{}@{}:5432/rustwebdev",
        user.trim(),
        password.trim(),
        host.trim(),
    );
    let qabase = QABase::new(&url).await;
    sqlx::migrate!()
        .run(&qabase.clone().connection)
        .await
        .expect("Cannot migrate DB");

    let appstate = AppState::new(qabase);
    let state = Arc::new(RwLock::new(appstate));

    let app = Router::new()
        .route("/api-docs", get(openapi))
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .route("/question", post(add_question))
        .route("/update", put(update_question))
        .route("/delete", delete(delete_question))
        .route("/add", post(add_answer))
        .with_state(state);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
