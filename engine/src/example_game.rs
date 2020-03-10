use nico_types::*;

pub fn game() -> Game {
    Game {
        obj_types: vec![ObjType(
            0,
            ObjTypeDetails::Sprite(Sprite {
                image: Image::load_png(include_bytes!("../static/img.png")).unwrap(),
            }),
        )],
        obj_instances: vec![ObjInstance {
            id: 0,
            type_id: 0,
            x: 10,
            y: 10,
        }],
        events: vec![
            Event(
                vec![Condition::System(SystemCondition::EveryTick)],
                vec![Action::Obj(
                    0,
                    ObjAction::General(GeneralObjAction::AddY(1)),
                )],
            ),
            Event(
                vec![Condition::System(SystemCondition::KeyPressed(
                    Key::Control(ControlKey::ArrowRight),
                ))],
                vec![Action::Obj(
                    0,
                    ObjAction::General(GeneralObjAction::AddX(1)),
                )],
            ),
            Event(
                vec![Condition::System(SystemCondition::KeyPressed(
                    Key::Control(ControlKey::ArrowLeft),
                ))],
                vec![Action::Obj(
                    0,
                    ObjAction::General(GeneralObjAction::AddX(-1)),
                )],
            ),
        ],
    }
}
