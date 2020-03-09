use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use nico_types::util;
use nico_types::{ObjType, ObjTypeDetails};

mod example_game;

struct Game {
    game: nico_types::Game,
}

impl Game {
    fn new(game: nico_types::Game) -> Self {
        Game { game }
    }

    fn render(&self) {
        let ctx = ctx();
        for inst in &self.game.obj_instances {
            let ObjType(_, ref ty) = self.game.obj_types[inst.type_id];
            match ty {
                ObjTypeDetails::Sprite(sp) => {
                    let im = sp.image.to_img_data();
                    ctx.put_image_data(&im, inst.x as f64, inst.y as f64)
                        .unwrap();
                }
                ObjTypeDetails::Text(_t) => todo!(),
            }
        }
    }
}

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

    let game = Game::new(example_game::game());

    game.render();

    Ok(())
}
