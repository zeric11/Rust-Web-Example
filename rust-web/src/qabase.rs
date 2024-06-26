use crate::*;

#[derive(Debug)]
pub enum DBError {
    //ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

#[derive(Debug, Clone)]
pub struct QABase {
    pub connection: PgPool,
}

impl QABase {
    /// Creates a new QABase connected to the supplied database URL
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        QABase {
            connection: db_pool,
        }
    }

    /// Retrieves questions stored in the database
    pub async fn get_questions(&self) -> Result<Vec<Question>, DBError> {
        match sqlx::query("SELECT * from questions;")
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }

    /// Retrieves a question stored in the database by ID
    pub async fn get_question(&self, question_id: &str) -> Result<Question, DBError> {
        let query = format!("SELECT * FROM questions WHERE id = {};", question_id);
        match sqlx::query(&query)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }

    /// Adds a question to the database
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, DBError> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
                 VALUES ($1, $2, $3)
                 RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }

    /// Updates a question in the database according to the supplied question ID
    pub async fn update_question(
        &self,
        question: Question,
        question_id: &str,
    ) -> Result<Question, DBError> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3
        WHERE id = $4
        RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }

    /// Deletes a question in the database according to the supplied question ID
    pub async fn delete_question(&self, question_id: &str) -> Result<bool, DBError> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }

    /// Adds an answer to the database
    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, DBError> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(DBError::DatabaseQueryError)
            }
        }
    }
}

pub struct AppState {
    pub qabase: QABase,
}

pub type HandlerAppState = State<Arc<RwLock<AppState>>>;

impl AppState {
    pub fn new(qabase: QABase) -> Self {
        Self { qabase }
    }
}
