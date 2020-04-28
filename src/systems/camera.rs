
use amethyst::{
    core::{Parent, transform::Transform},
    prelude::*,
    renderer::Camera,
    ecs::Entity
};
use crate::config::LevelConfig;
use crate::game::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub fn initialize_camera(world: &mut World, level_config: &LevelConfig, player: Entity) { 
    let mut transform = Transform::default();
    let half_tile_size = TILE_SIZE / 2.0;
    transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}