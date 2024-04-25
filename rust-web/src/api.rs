use crate::*;

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
#[debug_handler]
pub async fn get_questions(State(qabase): State<Arc<RwLock<QABase>>>) -> Response {
    let unpacked_qabase = qabase.read().await;
    let questions: Vec<Question> = unpacked_qabase
        .questions
        .read()
        .await
        .values()
        .cloned()
        .collect();

    (StatusCode::OK, Json(&questions)).into_response()
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
    State(qabase): State<Arc<RwLock<QABase>>>,
    Json(question): Json<Question>,
) -> Response {
    let unpacked_qabase = qabase.read().await;
    unpacked_qabase
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    StatusCode::OK.into_response()
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
    State(qabase): State<Arc<RwLock<QABase>>>,
    Json(question): Json<Question>,
) -> Response {
    let unpacked_qabase = qabase.read().await;
    match unpacked_qabase
        .questions
        .write()
        .await
        .get_mut(&question.id)
    {
        Some(q) => *q = question.clone(),
        None => return StatusCode::BAD_REQUEST.into_response(),
    }

    StatusCode::OK.into_response()
}

/// Delete question by id
#[utoipa::path(
    delete,
    path = "/api/v1/delete",
    responses(
        (status = 200, description = "Delete question", body = [Question])
    )
)]
#[debug_handler]
pub async fn delete_question(
    State(qabase): State<Arc<RwLock<QABase>>>,
    Path(id): Path<String>,
) -> Response {
    match qabase
        .read()
        .await
        .questions
        .write()
        .await
        .remove(&QuestionId(id))
    {
        Some(_) => return StatusCode::OK.into_response(),
        None => return StatusCode::BAD_REQUEST.into_response(),
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
    State(qabase): State<Arc<RwLock<QABase>>>,
    Json(answer): Json<Answer>,
) -> Response {
    let unpacked_qabase = qabase.read().await;
    unpacked_qabase
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    StatusCode::OK.into_response()
}
