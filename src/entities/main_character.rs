use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct MainCharacter ;

impl Component for MainCharacter {
    type Storage = DenseVecStorage<Self>;
}