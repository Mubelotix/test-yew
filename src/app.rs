use yew::prelude::*;
use yew_template::template_html;

use crate::log;

pub struct App {
    token: Option<String>,
}

pub enum Msg {
    Accept,
    Reject,
    GameCreated(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link2 = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let token = crate::api::create_game(String::from("mubelotix")).await;
            link2.send_message(Msg::GameCreated(token));
        });
        App {
            token: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Accept => {
                true
            },
            Msg::Reject => {
                true
            },
            Msg::GameCreated(token) => {
                log!("Game created: {}", token);
                self.token = Some(token);
                true
            },
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        template_html! {
            "src/main.html",
            title = "Yew Template",
            description = "A template for Yew projects",
            onclick_accept = { ctx.link().callback(|_| Msg::Accept) },
            onclick_reject = { ctx.link().callback(|_| Msg::Reject) },
            ...
        }
    }
}
