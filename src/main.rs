mod app;
mod api;
use app::App;
use api::*;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

fn main() {
    yew::Renderer::<App>::new().render();
}
