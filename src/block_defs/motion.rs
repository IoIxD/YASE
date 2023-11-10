use proc::block_derive;

use crate::{
    blocks::{Block, Value},
    from_fn_from_map,
};

#[block_derive]
#[derive(Debug, Clone)]
pub struct Move {
    pub(crate) steps: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct RotateLeft {
    pub(crate) degrees: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct RotateRight {
    pub(crate) degrees: Option<Value>,
}
#[derive(Debug, Clone)]
pub enum MovementOption {
    RandomPosition,
    MousePointer,
}
from_fn_from_map!(MovementOption, {
    "_random_" => RandomPosition,
    "_mouse_" => MousePointer,
});
#[derive(Debug, Clone)]
pub enum Goto {
    Pos(GotoPos),
    Option(GotoOption),
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct GotoPos {
    pub(crate) x: Option<Value>,
    pub(crate) y: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct GotoOption {
    pub(crate) option: Option<MovementOption>,
}
#[derive(Debug, Clone)]
pub enum Glide {
    Pos(GlidePos),
    Option(GlideOption),
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct GlidePos {
    pub(crate) x: Option<Value>,
    pub(crate) y: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct GlideOption {
    pub(crate) option: Option<MovementOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct PointTowardsMenu {
    pub(crate) option: Option<Point>,
}
#[derive(Debug, Clone)]
pub enum Point {
    Direction(PointDirection),
    Towards(PointOption),
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct PointDirection {
    pub(crate) x: Option<Value>,
    pub(crate) y: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct PointOption {
    pub(crate) option: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeX {
    pub(crate) x: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetX {
    pub(crate) x: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeY {
    pub(crate) y: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetY {
    pub(crate) y: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct IfOnEdgeBounce {}
#[derive(Debug, Clone)]
pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}
from_fn_from_map!(RotationStyle, {
    "left-right" => LeftRight,
    "don't rotate" => DontRotate,
    "all around" => AllAround,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetRotationStyle {
    pub(crate) style: Option<RotationStyle>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct XPosition {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct YPosition {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Direction {}
