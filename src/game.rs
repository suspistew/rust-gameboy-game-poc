use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let test = read_level();
        println!("{:?}", test);
    }
}

#[derive(Debug, Deserialize)]
struct LevelConfig {
    name: String,
    layers: HashMap<String, String>,
    character: PositionConfig,
    camera: PositionConfig,
}

#[derive(Debug, Deserialize)]
struct PositionConfig {
    x: f32,
    y: f32
}

fn read_level() -> LevelConfig{
    let input_path = format!("{}/assets/levels/level1.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let res :LevelConfig = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };
    res
}