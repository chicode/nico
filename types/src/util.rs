use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no window; where are you running this?")
}
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("no document; what did you even do?")
}

pub fn canvas_ctx(canv: web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canv.get_context("2d").unwrap().unwrap().dyn_into().unwrap()
}

pub fn create_ctx(w: u32, h: u32) -> web_sys::CanvasRenderingContext2d {
    let canv = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canv.set_width(w);
    canv.set_height(h);
    canvas_ctx(canv)
}
