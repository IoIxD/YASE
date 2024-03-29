#![allow(dead_code)]

use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use serde_json::Value;
/// This module contains the structure of a Scratch project and
/// the functions for interacting with it.
use std::{collections::HashMap, fs::read_to_string};

use crate::blocks;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Project {
    #[serde(rename = "targets")]
    #[serde(default)]
    sprites: Vec<Sprite>,
    #[serde(default)]
    extensions: Vec<String>,

    #[serde(default)]
    cur: usize,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Sprite {
    #[serde(default)]
    pub is_stage: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
    #[serde(default)]
    pub lists: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub broadcasts: HashMap<String, String>,
    #[serde(default)]
    pub blocks: HashMap<String, blocks::BlockType>,
    #[serde(default)]
    pub current_costume: f32,
    #[serde(default)]
    pub costumes: Vec<Costume>,
    #[serde(default)]
    pub sounds: Vec<Sound>,
    #[serde(default)]
    pub volume: f32,
    #[serde(default)]
    pub layer_order: f32,
    #[serde(default)]
    pub tempo: f32,
    #[serde(default)]
    pub video_transparency: f32,
    #[serde(default)]
    pub video_state: f32,
    #[serde(default)]
    pub tts_language: String,
    #[serde(default)]
    pub position_x: f32,
    #[serde(default)]
    pub position_y: f32,
    #[serde(default)]
    pub size: f32,
    #[serde(default)]
    pub direction: f32,
    #[serde(default)]
    pub draggable: bool,
    #[serde(default)]
    pub rotation_style: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Costume {
    #[serde(rename = "assetId")]
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
    #[serde(rename = "assetId")]
    asset_id: String,
    name: String,
    #[serde(rename = "dataFormat")]
    data_format: String,
    rate: f32,
    #[serde(rename = "sampleCount")]
    sample_count: f32,
    #[serde(rename = "md5ext")]
    md5: String,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    value: Value,
}

impl Project {
    pub fn new(id: Option<i32>) -> Result<Project, String> {
        let json: String = match id {
            Some(a) => {
                match reqwest::blocking::get(format!("https://projects.scratch.mit.edu/{}", a)) {
                    Ok(a) => match a.text() {
                        Ok(a) => a,
                        Err(err) => {
                            return Err(format!("error reading recieved json to string: {}", err));
                        }
                    },
                    Err(err) => {
                        return Err(format!("error getting json: {}", err));
                    }
                }
            }
            None => match read_to_string("./test.json") {
                Ok(a) => a,
                Err(err) => {
                    return Err(format!("error reading default file: {}", err));
                }
            },
        };

        // base project
        let project: Project = match serde_json::from_str(&json) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!("error unmarshalling json to project: {}", err));
            }
        };

        Ok(project)
    }

    pub fn extensions(&self) -> &Vec<String> {
        return &self.extensions;
    }
}

impl Iterator for Project {
    type Item = Sprite;

    fn next(&mut self) -> Option<Self::Item> {
        let o = match self.sprites.get(self.cur) {
            Some(a) => Some(a.clone()),
            None => None,
        };
        self.cur += 1;
        o
    }
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
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
