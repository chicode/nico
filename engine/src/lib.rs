use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

use nico_types::util;
use nico_types::{
    Action, Condition, Event, GeneralObjAction, ObjAction, ObjType, ObjTypeDetails, SystemAction,
    SystemCondition,
};

mod example_game;

#[wasm_bindgen(module = "/src/keys.js")]
extern "C" {
    #[wasm_bindgen(js_name = keyPressed)]
    fn key_pressed(k: &str) -> bool;
    #[wasm_bindgen(js_name = keyReleased)]
    fn key_released(k: &str) -> bool;
}

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

    pub fn step(&mut self) {
        for Event(conds, actions) in &self.game.events {
            if conds.iter().all(|c| self.eval_condition(c)) {
                for a in actions {
                    match a {
                        Action::System(a) => match a {
                            SystemAction::Terminate => panic!("terminate"),
                        },
                        Action::Obj(obj, a) => match a {
                            ObjAction::General(a) => match a {
                                GeneralObjAction::AddX(x) => self.game.obj_instances[*obj].x += *x,
                                GeneralObjAction::AddY(y) => self.game.obj_instances[*obj].y += *y,
                                GeneralObjAction::SetX(x) => self.game.obj_instances[*obj].x = *x,
                                GeneralObjAction::SetY(y) => self.game.obj_instances[*obj].y = *y,
                            },
                            ObjAction::Text(_t) => todo!(),
                        },
                    }
                }
            }
        }
    }

    fn eval_condition(&self, cond: &Condition) -> bool {
        match cond {
            Condition::System(c) => match c {
                SystemCondition::EveryTick => true,
                SystemCondition::KeyPressed(k) => key_pressed(k.to_code().as_ref()),
                SystemCondition::KeyReleased(k) => key_released(k.to_code().as_ref()),
            },
            Condition::Obj(_obj, _c) => todo!(),
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

    fn set_animation_frame(mut game: Game) {
        let f = move || {
            game.step();
            game.render();
            set_animation_frame(game)
        };
        let f = Closure::once_into_js(f);
        util::window()
            .request_animation_frame(f.unchecked_ref())
            .unwrap();
    }

    game.render();

    set_animation_frame(game);

    Ok(())
}
