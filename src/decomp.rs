#[allow(unused_variables)]
#[allow(dead_code)]

// Decompiler for Scratch games that Rust can understand and translate.

use std::{collections::HashMap, fs::read_to_string};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Project<'a>{
    #[serde(rename = "targets")]
    #[serde(default)]
    sprites: Vec<Sprite<'a>>,
    #[serde(default)]
    extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Sprite<'a>{
    #[serde(default)]
    is_stage: bool,
    #[serde(default)]
    name: String,
    #[serde(default)]
    variables: HashMap<String, Variable>,
    #[serde(default)]
    lists: HashMap<String, Vec<Value>>,
    #[serde(default)]
    broadcasts: HashMap<String, String>,
    #[serde(default)]
    blocks: HashMap<String, Block<'a>>,
    #[serde(default)]
    current_costume: f32,
    #[serde(default)]
    costumes: Vec<Costume>,
    #[serde(default)]
    sounds: Vec<Sound>,
    #[serde(default)]
    volume: f32,
    #[serde(default)]
    layer_order: f32,
    #[serde(default)]
    tempo: f32,
    #[serde(default)]
    video_transparency: f32,
    #[serde(default)]
    video_state: f32,
    #[serde(default)]
    tts_language: String,
    #[serde(default)]
    position_x: f32,
    #[serde(default)]
    position_y: f32,
    #[serde(default)]
    size: f32,
    #[serde(default)]
    direction: f32,
    #[serde(default)]
    draggable: bool,
    #[serde(default)]
    rotation_style: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Costume {
    #[serde(default)]
    asset_id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    bitmap_resolution: String,
    #[serde(default)]
    md5: String,
    #[serde(default)]
    data_format: String,
    #[serde(default)]
    rotation_center_x: String,
    #[serde(default)]
    rotation_center_y: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Sound {
    asset_id: String,
    name: String,
    data_format: String,
    rate: f32,
    sample_count: f32,
    md5: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Block<'a>{
    opcode: String,
    #[serde(rename = "next")]
    next_block_pointer: Option<String>,
    #[serde(rename = "parent")]
    parent_block_pointer: Option<String>,

    #[serde(skip_deserializing)]
    next_block: Option<&'a Block<'a>>,
    #[serde(skip_deserializing)]
    parent_block: Option<&'a Block<'a>>,

    #[serde(default)]
    inputs: HashMap<String, Value>,
    #[serde(default)]
    fields: HashMap<String, Value>,
    #[serde(default)]
    shadow: bool,
    #[serde(default)]
    top_level: bool,
    #[serde(default)]
    parent: Sprite<'a>,
}

impl<'a> Block<'a> {
    fn set_parent(&mut self, parent: &'a Block) {
        self.parent_block = Some(parent);
    }
    fn set_next(&mut self, next: &'a Block) {
        self.next_block = Some(next);
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    value: Value,
}

impl<'a> Project<'a> {
    pub fn new(id: Option<i32>) -> Result<Project<'a>, String> {
        let json: String = match id {
            Some(a) => {
                match reqwest::blocking::get(
                    format!("https://projects.scratch.mit.edu/{}",a)
                ) {
                    Ok(a) => match a.text() {
                        Ok(a) => a,
                        Err(err) => {
                            return Err(format!("error reading recieved json to string: {}",err));
                        }
                    },
                    Err(err) => {return Err(format!("error getting json: {}",err));}
                }
            },
            None => {
                match read_to_string("./test.json") {
                    Ok(a) => a,
                    Err(err) => {return Err(format!("error reading default file: {}",err));}
                }
            }
        };

        // base project
        let project: Project = match serde_json::from_str(&json) {
            Ok(a) => a,
            Err(err) => {return Err(format!("error unmarshalling json to project: {}",err));}
        };

        Ok(project)
    }
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>{
        let v: Vec<Value> = Vec::deserialize(d)?;
        let mut vi = v.into_iter();
        let name = vi.next();
        let value = vi.next();
        let fuck: Variable = Variable {
            // doing this without unwrap isn't possible because rust
            // complains about referencing a which falls out of scope
            name: name.unwrap().to_string().replace("\"", ""),
            value: value.unwrap(),
        };
        return Ok(fuck);
    }
}

