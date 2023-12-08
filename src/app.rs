use yew::prelude::*;
use yew_template::template_html;

pub struct App {

}

impl Component for App {
    type Message = ();
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
            ...
        }
    }
}
