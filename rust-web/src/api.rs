use crate::*;

use axum::response::{IntoResponse, Redirect, Response};

use qabase::*;
use qna::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_questions,
        add_question,
        update_question,
        delete_question,
        add_answer,
    ),
    components(
        schemas(Question)
    ),
    tags(
        (name = "knock-knock", description = "Knock-Knock Joke API")
    )
)]
pub struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// Return JSON of questions
#[utoipa::path(
    get,
    path = "/api/v1/questions",
    responses(
        (status = 200, description = "List questions", body = [Question])
    )
)]
pub async fn get_questions(State(appstate): HandlerAppState) -> Response {
    match appstate.read().await.qabase.get_questions().await {
        Ok(questions) => Json(questions).into_response(),
        Err(e) => StatusCode::BAD_REQUEST.into_response(),
    }
}

/// Post question JSON
#[utoipa::path(
    post,
    path = "/api/v1/question",
    responses(
        (status = 200, description = "Add question", body = [Question])
    )
)]
pub async fn add_question(
    State(appstate): State<Arc<RwLock<AppState>>>,
    Json(question): Json<NewQuestion>,
) -> Response {
    match appstate.read().await.qabase.add_question(question).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => StatusCode::BAD_REQUEST.into_response(),
    }
}

/// Put new content into question by id
#[utoipa::path(
    put,
    path = "/api/v1/update",
    responses(
        (status = 200, description = "Update question", body = [Question])
    )
)]
pub async fn update_question(
    State(appstate): State<Arc<RwLock<AppState>>>,
    Path(question_id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match appstate
        .read()
        .await
        .qabase
        .update_question(question, &question_id)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => StatusCode::BAD_REQUEST.into_response(),
    }
}

/// Delete question by id
#[utoipa::path(
    delete,
    path = "/api/v1/delete",
    responses(
        (status = 200, description = "Delete question", body = [Question])
    )
)]
pub async fn delete_question(
    State(appstate): State<Arc<RwLock<AppState>>>,
    Path(question_id): Path<String>,
) -> Response {
    match appstate
        .read()
        .await
        .qabase
        .delete_question(&question_id)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => StatusCode::BAD_REQUEST.into_response(),
    }
}

/// Add answer linked to question
#[utoipa::path(
    post,
    path = "/api/v1/answer",
    responses(
        (status = 200, description = "Add answer", body = [Question])
    )
)]
pub async fn add_answer(
    State(appstate): State<Arc<RwLock<AppState>>>,
    Json(answer): Json<NewAnswer>,
) -> Response {
    match appstate.read().await.qabase.add_answer(answer).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => StatusCode::BAD_REQUEST.into_response(),
    }
}
