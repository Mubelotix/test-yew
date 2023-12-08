use yew::prelude::*;
use yew_template::template_html;

pub struct App {

}

pub enum Msg {
    Accept,
    Reject
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {

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
