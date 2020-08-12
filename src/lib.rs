#![recursion_limit = "256"]

extern crate wasm_bindgen;
extern crate yew;

mod app;
mod components;
mod props;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
