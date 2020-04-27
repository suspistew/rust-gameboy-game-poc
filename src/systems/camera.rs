
use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};
use crate::config::LevelConfig;
use crate::game::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub fn initialize_camera(world: &mut World, level_config: &LevelConfig) { 
    let mut transform = Transform::default();
    let half_tile_size = TILE_SIZE / 2.0;
    transform.set_translation_xyz(
        (SCREEN_WIDTH / 2.0) - half_tile_size + level_config.camera.x * TILE_SIZE, 
        (SCREEN_HEIGHT / 2.0 ) - half_tile_size + level_config.camera.y * TILE_SIZE,
        1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}