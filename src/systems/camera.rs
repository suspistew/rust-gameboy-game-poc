use crate::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use amethyst::{
    core::{transform::Transform, Parent},
    ecs::Entity,
    prelude::*,
    renderer::Camera,
};

pub fn initialize_camera(world: &mut World, player: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}
