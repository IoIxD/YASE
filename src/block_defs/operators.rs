use proc::block_derive;

use crate::blocks::{Block, Value};

#[block_derive]
#[derive(Debug, Clone)]
pub struct Add {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Sub {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Mul {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Divide {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct PickRandom {
    pub(crate) min: Option<Value>,
    pub(crate) max: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct GreaterThen {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct LesserThen {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct EqualTo {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct And {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Or {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Not {
    pub(crate) a: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Join {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct LetterOf {
    pub(crate) index: Option<Value>,
    pub(crate) a: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct LengthOf {
    pub(crate) a: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Contains {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Modulo {
    pub(crate) a: Option<Value>,
    pub(crate) b: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Round {
    pub(crate) a: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Absolute {
    pub(crate) a: Option<Value>,
}
