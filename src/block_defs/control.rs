use proc::block_derive;

use crate::blocks::{Block, Value};

#[block_derive]
#[derive(Debug, Clone)]
pub struct WaitSeconds {
    pub(crate) seconds: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Repeat {
    pub(crate) units: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Forever {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct IfThen {
    pub(crate) condition: Option<Value>,
    pub(crate) then: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct IfThenElse {
    pub(crate) condition: Option<Value>,
    pub(crate) then: Option<Value>,
    pub(crate) otherwise: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WaitUntil {
    pub(crate) condition: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct RepeatUntil {
    pub(crate) condition: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct StopAll {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct WhenIStartAsAClone {}
#[derive(Debug, Clone)]
pub enum SpriteOption {
    Myself,
    Sprite(String),
}
impl SpriteOption {
    pub fn from(val: Option<Value>) -> Option<SpriteOption> {
        match val {
            Some(Value::String(a)) => match a.as_str() {
                "_myself_" => Some(Self::Myself),
                _ => Some(Self::Sprite(a)),
            },
            _ => {
                #[cfg(debug_assertions)]
                panic!("invalid sprite option: {:?}", val);
                #[cfg(not(debug_assertions))]
                None
            }
        }
    }
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct CreateCloneOf {
    pub(crate) of: Option<SpriteOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DeleteClone {}
