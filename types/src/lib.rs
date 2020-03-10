use serde::{Deserialize, Serialize};

mod img;
mod keys;
pub mod util;

pub use img::Image;
pub use keys::{ControlKey, Key};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub obj_types: Vec<ObjType>,
    pub obj_instances: Vec<ObjInstance>,
    pub events: Vec<Event>,
}

// Objects

// Object Types

pub type ObjTypeId = usize;

#[derive(Serialize, Deserialize)]
pub struct ObjType(pub ObjTypeId, pub ObjTypeDetails);

#[derive(Serialize, Deserialize)]
pub enum ObjTypeDetails {
    Sprite(Sprite),
    Text(Text),
}

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub text: String,
}

// Object Instances

pub type ObjInstanceId = usize;

#[derive(Serialize, Deserialize)]
pub struct ObjInstance {
    pub id: ObjInstanceId,
    pub type_id: ObjTypeId,
    pub x: isize,
    pub y: isize,
}

// Events

#[derive(Serialize, Deserialize)]
pub struct Event(pub Vec<Condition>, pub Vec<Action>);

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
    AddX(isize),
    AddY(isize),
    SetX(isize),
    SetY(isize),
}

#[derive(Serialize, Deserialize)]
pub enum TextAction {
    SetText(String),
}
