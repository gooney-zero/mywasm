
#![recursion_limit = "1024"]
pub mod app;
pub mod routes;
pub mod pages;
pub mod components;
use wasm_bindgen::prelude::*;
use yew::App;


#[wasm_bindgen]
pub fn run_app() {
    // yew::start_app::<app::App>();
    App::<app::AppRoot>::new().mount_to_body();
    // Ok(())
}
