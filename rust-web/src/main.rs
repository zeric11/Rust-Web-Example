mod api;
mod qabase;
mod qna;

use api::*;
use qabase::*;
use qna::*;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use std::net::SocketAddr;
use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Redirect, Response, Result},
    routing::{delete, get, post, put},
    Json, Router,
};

use sqlx::{
    self,
    postgres::{PgConnection, PgPool, PgPoolOptions, PgRow, Postgres},
    Pool, Row,
};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
extern crate serde_json;
extern crate thiserror;
use tokio::{self, sync::RwLock};
use tower_http::{services, trace};
extern crate tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{
    openapi::schema::{ObjectBuilder, Schema, SchemaType},
    openapi::RefOr,
    OpenApi, ToSchema,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let user = "rustweb";
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
