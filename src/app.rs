use yew::prelude::*;
use yew_template::template_html;

use crate::{log, api::{GameState, Measure}};

pub struct App {
    token: Option<String>,
    state: GameState,
    measure: Measure,
}

pub enum Msg {
    Accept,
    Reject,
    GameCreated(String),
    StateUpdated(GameState),
    MeasureUpdated(Measure),
}

impl App {
    fn update(&mut self, ctx: &Context<Self>) {
        let token2 = self.token.as_ref().unwrap().clone();
        let token3 = token2.clone();
        let link2 = ctx.link().clone();
        let link3 = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let game_state = crate::api::get_game(&token2).await;
            link2.send_message(Msg::StateUpdated(game_state));
        });
        wasm_bindgen_futures::spawn_local(async move {
            let measure = crate::api::get_measure(&token3).await;
            link3.send_message(Msg::MeasureUpdated(measure));
        });
    }
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
            state: GameState::default(),
            measure: Measure::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Accept => {
                let token = self.token.as_ref().unwrap().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    crate::api::accept(&token).await;
                });
                self.update(ctx);
                true
            },
            Msg::Reject => {
                let token = self.token.as_ref().unwrap().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    crate::api::reject(&token).await;
                });
                self.update(ctx);
                true
            },
            Msg::GameCreated(token) => {
                log!("Game created: {}", token);
                self.token = Some(token);
                self.update(ctx);
                true
            },
            Msg::MeasureUpdated(measure) => {
                log!("Measure updated: {:?}", measure);
                self.measure = measure;
                true
            },
            Msg::StateUpdated(game_state) => {
                log!("Game updated: {:?}", game_state);
                self.state = game_state;
                true
            },
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        template_html! {
            "src/main.html",
            title = { self.measure.title.clone() },
            description = { self.measure.description.clone() },
            social_progress = { self.state.social.to_string() },
            environmental_progress = { self.state.environmental.to_string() },
            economic_progress = { self.state.economic.to_string() },
            scientists_progress = { self.state.scientist.to_string() },
            united_nations_progress = { self.state.united_nations.to_string() },
            cartel_progress = { self.state.cartel.to_string() },
            gameover = { self.state.game_over },
            current_month = { match self.state.current_month {
                1 => "janvier",
                2 => "février",
                3 => "mars",
                4 => "avril",
                5 => "mai",
                6 => "juin",
                7 => "juillet",
                8 => "août",
                9 => "septembre",
                10 => "octobre",
                11 => "novembre",
                _ => "décembre",
            } },
            current_year = { self.state.current_year },
            onclick_accept = { ctx.link().callback(|_| Msg::Accept) },
            onclick_reject = { ctx.link().callback(|_| Msg::Reject) },
            ...
        }
    }
}
