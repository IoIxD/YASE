#[allow(unused_variables)]
#[allow(dead_code)]

// Decompiler for Scratch games that Rust can understand and translate.

use std::{collections::HashMap, fs::read_to_string, fmt, marker::PhantomData};
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct Project{
    #[serde(rename = "targets")]
    #[serde(default)]
    objects: Vec<Object>,
    #[serde(default)]
    extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Object{
    #[serde(default)]
    is_stage: bool,
    #[serde(default)]
    name: String,
    #[serde(default)]
    variables: HashMap<String, Value>,
    #[serde(default)]
    lists: HashMap<String, Vec<Value>>,
    #[serde(default)]
    broadcasts: HashMap<String, String>,
    #[serde(default)]
    blocks: HashMap<String, Block>,
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

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize, Default)]
pub struct Sound {
    asset_id: String,
    name: String,
    data_format: String,
    rate: f32,
    sample_count: f32,
    md5: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Block{
    opcode: String,
    #[serde(rename = "next")]
    next_block_pointer: Option<String>,
    #[serde(rename = "parent")]
    parent_block_pointer: Option<String>,
    #[serde(default)]
    inputs: HashMap<String, Value>,
    #[serde(default)]
    fields: HashMap<String, Value>,
    #[serde(default)]
    shadow: bool,
    #[serde(default)]
    top_level: bool,
    #[serde(default)]
    parent: Object,
}

impl Project {
    pub fn new(id: Option<i32>) -> Result<Project, String> {
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
        let project: Project = match serde_json::from_str(&json) {
            Ok(a) => a,
            Err(err) => {return Err(format!("error unmarshalling json to project: {}",err));}
        };
        Ok(project)
    }
}