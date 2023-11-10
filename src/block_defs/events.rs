use proc::block_derive;

use crate::{
    blocks::{Block, Value},
    from_fn_from_map,
};

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenGreenFlagClicked {}
#[derive(Debug, Clone)]
pub enum Key {
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    Space,
    Any,
    /// Any character from a-z and 0-9
    Alphanumerical(char),
}
impl Key {
    pub fn from(val: Option<Value>) -> Option<Key> {
        match val {
            Some(a) => match a {
                Value::Number(_) => todo!(),
                Value::String(a) => match a.as_str() {
                    "up arrow" => Some(Self::UpArrow),
                    "down arrow" => Some(Self::DownArrow),
                    "right arrow" => Some(Self::RightArrow),
                    "left arrow" => Some(Self::LeftArrow),
                    "space" => Some(Self::Space),
                    "any" => Some(Self::Any),
                    _ => Some(Self::Alphanumerical(a.chars().next().unwrap())),
                },
                Value::Null => todo!(),
            },
            _ => None,
        }
    }
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenKeyPressed {
    pub key: Option<Key>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenSpriteClicked {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenStageClicked {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenBackdropSwitchesTo {
    pub backdrop: Option<Value>,
}
#[derive(Debug, Clone)]
pub enum EventOption {
    Loudness,
    Timer,
}
from_fn_from_map!(EventOption, {
    "loudness" => Loudness,
    "timer" => Timer,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenOptionGreaterThen {
    pub option: Option<EventOption>,
    pub by: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenIRecieveBroadcast {
    pub broadcast: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Broadcast {
    pub broadcast: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct BroadcastAndWait {
    pub broadcast: Option<Value>,
}
