use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, window};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Pseudo {
    pseudo: String,
}

pub async fn create_game(pseudo: String) -> String {
    let pseudo = Pseudo { pseudo };

    let mut req_init = web_sys::RequestInit::new();
    req_init.method("POST");
    req_init.body(Some(&JsValue::from_str(&serde_json::to_string(&pseudo).unwrap())));

    let request = Request::new_with_str_and_init("http://localhost:8000/create_game", &req_init).unwrap();
    //request.headers().set(
    //    "Api-Key",
    //    &format!("{}-{}-{}", api_key, counter, gen_code(api_key, counter)),
    //)?;
    request.headers().set("Content-Type", "application/json").expect("Failed to set content type");

    let response = JsFuture::from(window().unwrap().fetch_with_request(&request)).await.expect("Failed to fetch");
    let response: web_sys::Response = response.clone().dyn_into().unwrap();
    let text = JsFuture::from(response.text().unwrap()).await.unwrap();

    if response.status() != 200 {
        panic!("Failed to create game: {}", text.as_string().unwrap());
    }

    text.as_string().unwrap()
}
