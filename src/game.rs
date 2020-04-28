use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, Texture},
    ecs::Entity
};

use crate::config::{read_level, load_misc_spritesheet, load_character_spritesheet, LevelConfig};
use crate::systems::{initialize_camera};
use crate::entities::MainCharacter;

pub const SCREEN_HEIGHT: f32 = 288.0;
pub const SCREEN_WIDTH: f32 = 352.0;
pub const TILE_SIZE: f32 = 32.0;
pub const NO_TILE_ID: i32 = -1; 


pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let level_1 = read_level(1);
        let world = data.world;
        let misc_spritesheet_handle = load_misc_spritesheet(world);
        let character_spritesheet_handle = load_character_spritesheet(world);

        initialize_layer(world, &level_1, misc_spritesheet_handle.clone(), "background", 0.01);
        initialize_layer(world, &level_1, misc_spritesheet_handle.clone(), "misc", 0.03);
        let player = initialize_player(world, &level_1, character_spritesheet_handle.clone());
        initialize_camera(world, &level_1, player);
    }
}

fn initialize_layer (world: &mut World, level: &LevelConfig, sprite_sheet_handle: Handle<SpriteSheet>, layer: &str, layer_position: f32) {
    
    match level.layers.get(layer) {
        Some(sprites) => {
            let lines: Vec<_>  = sprites.split(';').collect();
            let line_nb = lines.len();
            for (y, line) in lines.iter().enumerate() {
                let tiles: Vec<_>  = line.split(',').collect();
                for (x, tile) in tiles.iter().enumerate(){
                    let (tile_x, tile_y) = (x, line_nb - y - 1);
                    
                    let tile_number: i32 = match tile.trim().parse(){
                        Ok(num) => num,
                        Err(_) => NO_TILE_ID
                    };

                    if tile_number != NO_TILE_ID && tile_number >= 0 {
                        let sprite_render = SpriteRender {
                            sprite_sheet: sprite_sheet_handle.clone(),
                            sprite_number: tile_number as usize,
                        };
    
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(tile_x as f32 * TILE_SIZE, tile_y as f32* TILE_SIZE, layer_position);
                        world
                            .create_entity()
                            .with(sprite_render)
                            .with(transform)
                            .build();
                    }
                }
            }
        },
        None => {
            println!("Impossible to find the layer {} in the level config", layer);
        }
    };
}

fn initialize_player (world: &mut World, level: &LevelConfig, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(level.character.x * TILE_SIZE, level.character.y as f32* TILE_SIZE, 0.02);
    world
        .create_entity()
        .with(sprite_render)
        .with(MainCharacter)
        .with(transform)
        .build()
}