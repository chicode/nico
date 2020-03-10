use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub enum Key {
    Char(char),
    Control(ControlKey),
    CustomCode(String),
}
#[derive(Serialize, Deserialize, strum::IntoStaticStr)]
pub enum ControlKey {
    Backspace,
    Delete,
    Insert,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
}

impl Key {
    pub fn to_code(&self) -> Cow<str> {
        match self {
            Self::Char(c) => format!("Key{}", c.to_uppercase()).into(),
            Self::Control(k) => Cow::Borrowed(k.into()),
            Self::CustomCode(s) => s.as_str().into()
        }
    }
}
