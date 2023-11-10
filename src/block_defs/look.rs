use proc::block_derive;

use crate::{
    blocks::{Block, Value},
    from_fn_from_map,
};

#[block_derive]
#[derive(Debug, Clone)]
pub struct SayForever {
    pub(crate) message: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Say {
    pub(crate) message: Option<Value>,
    pub(crate) secs: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Think {
    pub(crate) message: Option<Value>,
    pub(crate) secs: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ThinkForever {
    pub(crate) message: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SwitchCostume {
    pub(crate) costume: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SwitchCostumeAndWait {
    pub(crate) costume: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct NextCostume {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SwitchBackdrop {
    pub(crate) backdrop: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SwitchBackdropAndWait {
    pub(crate) backdrop: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct NextBackdrop {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeSize {
    pub(crate) units: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetSize {
    pub(crate) percentage: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ClearGraphicEffects {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ShowSprite {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct HideSprite {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct HideAllSprites {}
#[derive(Debug, Clone)]
pub enum LayerOption {
    Front,
    Back,
}
from_fn_from_map!(LayerOption, {
    "front" => Front,
    "back" => Back,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct GotoLayer {
    pub(crate) option: Option<LayerOption>,
}
#[derive(Debug, Clone)]
pub enum LayerDirection {
    Forward,
    Backward,
    Value(Value),
}
from_fn_from_map!(LayerDirection, {
    "forward" => Forward,
    "backward" => Backward,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeLayer {
    pub(crate) direction: Option<LayerDirection>,
    pub(crate) by: Option<Value>,
}
#[derive(Debug, Clone)]
pub enum Costume {
    ByNumber(CostumeByNumber),
    ByName(CostumeByName),
    WithName(Option<Value>),
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct CostumeByNumber {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct CostumeByName {}
#[derive(Debug, Clone)]
pub enum Backdrop {
    ByNumber(BackdropByNumber),
    ByName(BackdropByName),
    WithName(Option<Value>),
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct BackdropByNumber {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct BackdropByName {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Size {}
