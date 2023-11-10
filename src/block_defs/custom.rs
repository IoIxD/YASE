use proc::block_derive;

use crate::blocks::{Block, Value};

#[block_derive]
#[derive(Debug, Clone)]
pub struct ProceduresDefinition {
    pub(crate) block: String,
}

#[block_derive]
#[derive(Debug, Clone)]

pub struct ProceduresCall {}

#[block_derive]
#[derive(Debug, Clone)]

pub struct ProceduresPrototype {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ProceduresDeclaration {}
