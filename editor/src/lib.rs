#![recursion_limit = "512"]

mod app;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    yew::start_app::<app::App>();

    Ok(())
}
