use serde::{Deserialize, Serialize};

mod img;
pub mod util;

pub use img::Image;

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
    pub x: usize,
    pub y: usize,
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
    AddX(usize),
    AddY(usize),
    SetX(usize),
    SetY(usize),
}

#[derive(Serialize, Deserialize)]
pub enum TextAction {
    SetText(String),
}
