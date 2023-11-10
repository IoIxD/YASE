use std::collections::HashMap;

use proc::block_derive;
use serde::{de::Visitor, Deserialize};
use serde_derive::Deserialize;
use serde_json::Value as SerdeValue;

use crate::blocks::{Block, Value};

#[block_derive]
#[derive(Debug, Clone)]
pub struct ProceduresDefinition {
    pub(crate) block: String,
}

#[block_derive]
#[derive(Debug, Clone)]

pub struct ProceduresCall {}

/* "mutation":{"tagName":"mutation","proccode":"perlin %n %n","argumentnames":"[\"x\",\"y\"]","argumentids":"[\"input0\",\"input1\"]","argumentdefaults":"[1,1]","warp":true,"children":[] */
#[block_derive]
#[derive(Debug, Clone)]

pub struct ProceduresPrototype {
    pub(crate) mutation: Mutation,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct ProceduresDeclaration {}

#[derive(Deserialize, Debug, Clone)]
pub struct Mutation {
    #[serde(rename = "tagName")]
    pub(crate) tag_name: Option<SerdeValue>,
    pub(crate) proccode: Option<SerdeValue>,
    #[serde(rename = "argumentnames")]
    pub(crate) argument_names: Option<SerdeValue>,
    #[serde(rename = "argumentids")]
    pub(crate) argument_ids: Option<SerdeValue>,
    #[serde(rename = "argumentdefaults")]
    pub(crate) argument_defaults: Option<SerdeValue>,

    pub(crate) warp: bool,
    pub(crate) children: Option<SerdeValue>,
}

pub struct MutationVisitor;
impl<'de> Visitor<'de> for MutationVisitor {
    type Value = Mutation;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a mutation block")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut hashmap: HashMap<String, SerdeValue> = HashMap::new();
        #[allow(irrefutable_let_patterns)] // we handle breaking ourselves.
        while let k = map.next_entry() {
            match k {
                Ok(a) => match a {
                    Some(a) => {
                        hashmap.insert(a.0, a.1);
                    }
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(Mutation {
            tag_name: hashmap.get("tagName").cloned(),
            proccode: hashmap.get("proccode").cloned(),
            argument_names: hashmap.get("argumentnames").cloned(),
            argument_ids: hashmap.get("argumentids").cloned(),
            argument_defaults: hashmap.get("argumentdefaults").cloned(),
            warp: hashmap.get("warp").cloned().unwrap().as_bool().unwrap(),
            children: hashmap.get("children").cloned(),
        })
    }
}
