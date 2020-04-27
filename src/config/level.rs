use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub layers: HashMap<String, String>,
    pub character: PositionConfig,
    pub camera: PositionConfig,
}

#[derive(Debug, Deserialize)]
pub struct PositionConfig {
    pub x: f32,
    pub y: f32
}

pub fn read_level(lvl_number: usize) -> LevelConfig{
    let input_path = format!("{}/assets/levels/level_{}.ron", env!("CARGO_MANIFEST_DIR"), lvl_number);
    let f = File::open(&input_path).expect("Failed opening file");
    let res :LevelConfig = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load level {}: {}", lvl_number, e);
            std::process::exit(1);
        }
    };
    res
}