mod api;
mod qna;

use api::*;
use qna::*;

use std::collections::{HashMap, HashSet};
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
use axum_macros::debug_handler;
extern crate fastrand;
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
    let qabase = QABase::new();
    let qabase = Arc::new(RwLock::new(qabase));

    let app = Router::new()
        .route("/api-docs", get(openapi))
        .route("/questions", get(get_questions))
        .route("/question", post(add_question))
        .route("/update", put(update_question))
        .route("/delete", delete(delete_question))
        .route("/add", post(add_answer))
        .with_state(qabase);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
