mod finder;
mod qna;

use finder::*;
use qna::*;

use std::collections::HashSet;

extern crate serde;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub type QAResult = Result<QAStruct, gloo_net::Error>;

struct App {
    question: QAResult,
}

pub enum Msg {
    GotQA(QAResult),
    GetQA(Option<String>),
}

impl App {
    fn refresh_questions(ctx: &Context<Self>, question_id: Option<String>) {
        // All requests from the back-end are returning as "Failed to fetch" errors.
        let new_question = QAStruct::get_question(question_id);
        ctx.link().send_future(new_question);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_questions(ctx, None);
        let question = Err(gloo_net::Error::GlooError("Loading QnA…".to_string()));
        Self { question }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotQA(question) => {
                self.question = question;
                true
            }
            Msg::GetQA(question_id) => {
                App::refresh_questions(ctx, question_id);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let question = &self.question;
        html! {
            <>
                <h1>{ "QnA" }</h1>
                if let Ok(ref question) = question {
                    <Question question={question.clone()}/>
                }
                else if let Err(ref error) = question {
                    <div>
                        <span class="error">{format!("Server Error: {error}")}</span>
                    </div>
                }
                <Finder on_find={ctx.link().callback(Msg::GetQA)}/>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
