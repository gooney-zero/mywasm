// extern crate wasm_bindgen;

// mod components;
mod app;
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
