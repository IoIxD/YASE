use proc::block_derive;

use crate::{
    blocks::{Block, Value},
    from_fn_from_map,
};

use super::events::Key;

#[derive(Debug, Clone)]
pub enum SensingOption {
    MousePointer,
    Edge,
    Sprite(Value),
}
impl SensingOption {
    pub fn from(val: Option<Value>) -> Option<SensingOption> {
        match val {
            Some(Value::String(a)) => match a.as_str() {
                "_mouse_" => Some(Self::MousePointer),
                "_edge_" => Some(Self::Edge),
                _ => Some(Self::Sprite(Value::String(a))),
            },
            _ => None,
        }
    }
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Touching {
    pub(crate) touching: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct TouchingMenu {
    pub(crate) touching: Option<SensingOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct TouchingColor {
    pub(crate) color: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ColorTouchingColor {
    pub(crate) color1: Option<Value>,
    pub(crate) color2: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DistanceTo {
    pub(crate) to: Option<SensingOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Answer {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct KeyPressed {
    pub(crate) key: Option<Key>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct MouseDown {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct MouseX {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct MouseY {}
#[derive(Debug, Clone)]
pub enum DraggableOption {
    Draggable,
    NotDraggable,
}
from_fn_from_map!(DraggableOption, {
    "draggable" => Draggable,
    "not draggable" => NotDraggable,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetDragMode {
    pub(crate) option: Option<DraggableOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Loudness {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Timer {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ResetTimer {}
#[derive(Debug, Clone)]
pub enum BackdropOfOption {
    BackdropNumber,
    BackdropName,
    Volume,
    MyVariable,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct BackdropOf {
    pub(crate) backdrop: BackdropOfOption,
}
#[derive(Debug, Clone)]
pub enum CurrentTimeOption {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}
from_fn_from_map!(CurrentTimeOption, {
    "year" => Year,
    "month" => Month,
    "date" => Date,
    "day of week" => DayOfWeek,
    "hour" => Hour,
    "minute" => Minute,
    "second" => Second,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct CurrentTime {
    pub(crate) option: Option<CurrentTimeOption>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DaysSince2000 {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Username {}
