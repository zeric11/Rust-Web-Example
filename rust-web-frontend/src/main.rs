mod cookie;
mod finder;
mod qna;

use cookie::*;
use finder::*;
use qna::*;

use std::collections::HashSet;

extern crate serde;
// use gloo_console::log;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use wasm_cookies as cookies;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub type QAResult = Result<QAStruct, gloo_net::Error>;

struct App {
    cookie: String,
    questions: QAResult,
}

pub enum Msg {
    GotQA(QAResult),
    GetQA(Option<String>),
}

impl App {
    fn refresh_questions(ctx: &Context<Self>) {
        let got_questions = QAStruct::get_questions();
        ctx.link().send_future(got_questions);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cookie = acquire_cookie();
        App::refresh_questions(ctx);
        let questions = Err(gloo_net::Error::GlooError("Loading QnA…".to_string()));
        Self { cookie, questions }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotQA(questions) => {
                self.questions = questions;
                true
            }
            Msg::GetQA(_) => {
                App::refresh_questions(ctx);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cookie = &self.cookie;
        let questions = &self.questions;
        html! {
        <>
            <h1>{ "QnA" }</h1>
            if false {
                {render_cookie(cookie)}
            }
            if let Ok(ref questions) = questions {
                <Question questions={questions.clone()}/>
            }
            if let Err(ref error) = questions {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <div>
                <button onclick={ctx.link().callback(|_| Msg::GetQA(None))}>{"Tell me another!"}</button>
            </div>
            <Finder on_find={ctx.link().callback(Msg::GetQA)}/>
        </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
