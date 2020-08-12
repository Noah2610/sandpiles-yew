extern crate wasm_bindgen;
extern crate yew;

mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    eprintln!("!!!HELLO WORLD!!!");
    yew::start_app::<app::App>();
    Ok(())
}
