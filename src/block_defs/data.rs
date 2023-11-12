use proc::block_derive;

use crate::blocks::Value;
use crate::Block;

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataGetVariable {
    pub(crate) variable: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataSetVariableTo {
    pub(crate) variable: Value,
    pub(crate) value: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataChangeVariableBy {
    pub(crate) variable: Value,
    pub(crate) value: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataShowVariable {
    pub(crate) variable: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataHideVariable {
    pub(crate) variable: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataListContents {
    pub(crate) variable: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataListIndexAll {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataListIndexAllRandom {}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataAddToList {
    pub(crate) item: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataDeleteOfList {
    pub(crate) item: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataDeleteAllOfList {
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataInsertAtList {
    pub(crate) item: Value,
    pub(crate) index: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataReplaceItemOfList {
    pub(crate) item: Value,
    pub(crate) index: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataItemOfList {
    pub(crate) index: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataLengthOfList {
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataListContainsItem {
    pub(crate) input: Value,
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataShowList {
    pub(crate) list: Value,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct DataHideList {
    pub(crate) list: Value,
}
