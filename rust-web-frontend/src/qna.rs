use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QAStruct {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

impl QAStruct {
    pub async fn get_questions() -> Msg {
        let request = "http://localhost:3000/api/v1/questions".to_string();
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQA(Err(e)),
            Ok(data) => Msg::GotQA(data.json().await),
        }
    }
}
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QAProps {
    pub questions: QAStruct,
}

#[function_component(Question)]
pub fn questions(qna: &QAProps) -> Html {
    let questions = &qna.questions;
    html! { <>
        <div class="qna">
            <span class="question">{format!("Q: {}", questions.question.clone())}</span><br/>
            <span class="answer">{format!("A: {}", questions.answer.clone())}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &questions.id)}
            if let Some(ref tags) = questions.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            if let Some(ref source) = questions.source {
                {format!("; source: {}", source)}
            }
            {"]"}
        </span>
    </> }
}
