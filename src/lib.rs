extern crate wasm_bindgen;
extern crate yew;

mod app;
mod components;
mod props;
mod sandpiles;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
