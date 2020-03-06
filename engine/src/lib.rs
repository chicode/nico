use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use nico_types::util;

fn ctx() -> web_sys::CanvasRenderingContext2d {
    util::canvas_ctx(
        util::document()
            .get_element_by_id("canv")
            .unwrap()
            .dyn_into()
            .unwrap(),
    )
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut img = nico_types::Image::load_png(include_bytes!("../static/img.png")).unwrap();

    let img_data = img.to_img_data();

    ctx().put_image_data(&img_data, 0.0, 0.0)?;

    Ok(())
}
