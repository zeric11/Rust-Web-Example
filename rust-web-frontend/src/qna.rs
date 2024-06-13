use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QAStruct {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
}

impl QAStruct {
    pub async fn get_question(question_id: Option<String>) -> Msg {
        let request = match &question_id {
            // When no question ID is supplied, all of the questions should be returned from the database.
            // However, this feature is not yet implemented, so just the first question is requested from
            // the server for now.
            None => "http://localhost:3000/questions/0".to_string(),
            Some(ref question_id) => format!("http://localhost:3000/questions/{}", question_id),
        };
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
    pub question: QAStruct,
}

#[function_component(Question)]
pub fn question(question: &QAProps) -> Html {
    let question = &question.question;
    html! {
        <>
            <div class="qna">
                <span class="question">{format!("Q: {}", question.content.clone())}</span><br/>
            </div>
        </>
    }
}
