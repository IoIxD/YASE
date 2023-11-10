use proc::block_derive;

use crate::{
    blocks::{Block, Value},
    from_fn_from_map,
};

#[block_derive]
#[derive(Debug, Clone)]
pub struct PlaySound {
    pub(crate) sound: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct PlaySoundUntilDone {
    pub(crate) sound: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct StartSound {
    pub(crate) sound: String,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct StopAllSounds {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SoundEffectsMenu {
    pub(crate) option: Option<SoundEffect>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SoundSoundsMenu {
    pub(crate) option: Option<Value>,
}
#[derive(Debug, Clone)]
pub enum SoundEffect {
    Pitch,
    Pan,
}
from_fn_from_map!(SoundEffect, {
    "PITCH" => Pitch,
    "PAN" => Pan,
});

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeEffectBy {
    pub(crate) effect: Option<Value>,
    pub(crate) units: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetEffectTo {
    pub(crate) effect: Option<Value>,
    pub(crate) percentage: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ClearSoundEffects {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ChangeVolumeBy {
    pub(crate) units: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct SetVolumeTo {
    pub(crate) percentage: Option<Value>,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct Volume {}
