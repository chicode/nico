use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();


    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();

    let canv = doc.get_element_by_id("canv").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx = canv.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    Ok(())
}
