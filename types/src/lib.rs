use serde::{Deserialize, Serialize};

pub use image::RgbaImage as Image;

mod png_serde;

#[derive(Serialize, Deserialize)]
pub struct Game {
    obj_types: Vec<ObjType>,
    obj_instances: Vec<ObjInstance>,
    events: Vec<Event>,
}

// Objects

// Object Types

pub type ObjTypeId = usize;

#[derive(Serialize, Deserialize)]
pub struct ObjType(ObjTypeId, ObjTypeDetails);

#[derive(Serialize, Deserialize)]
pub enum ObjTypeDetails {
    Sprite(Sprite),
    Text(Text),
}

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    #[serde(with = "png_serde")]
    image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    text: String,
}

// Object Instances

pub type ObjInstanceId = usize;

#[derive(Serialize, Deserialize)]
pub struct ObjInstance {
    id: ObjInstanceId,
    type_id: ObjTypeId,
    x: usize,
    y: usize,
}

// Events

#[derive(Serialize, Deserialize)]
pub struct Event(Vec<Condition>, Vec<Action>);

// Conditions

#[derive(Serialize, Deserialize)]
pub enum Condition {
    System(SystemCondition),
    Obj(ObjInstanceId, ObjCondition),
}

#[derive(Serialize, Deserialize)]
pub enum SystemCondition {
    KeyPressed(Key),
    KeyReleased(Key),
    EveryTick,
}

pub type Key = u8;

#[derive(Serialize, Deserialize)]
pub enum ObjCondition {
    General(GeneralObjCondition),
}

#[derive(Serialize, Deserialize)]
pub enum GeneralObjCondition {
    CompareX(usize),
    CompareY(usize),
    CollidedWith(ObjInstanceId),
}

// Actions

#[derive(Serialize, Deserialize)]
pub enum Action {
    System(SystemAction),
    Obj(ObjInstanceId, ObjAction),
}

#[derive(Serialize, Deserialize)]
pub enum SystemAction {
    Terminate,
}

#[derive(Serialize, Deserialize)]
pub enum ObjAction {
    General(GeneralObjAction),
    Text(TextAction),
}

#[derive(Serialize, Deserialize)]
pub enum GeneralObjAction {
    SetX(usize),
    SetY(usize),
}

#[derive(Serialize, Deserialize)]
pub enum TextAction {
    SetText(String),
}
