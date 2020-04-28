use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender, Camera};

use crate::entities::MainCharacter;
use crate::game::TILE_SIZE;
use amethyst::core::timing::Time;

const TILE_MOVEMENT_DURATION: f32 = 0.4;

enum Orientation {
    UP, RIGHT, DOWN, LEFT
}


struct Movement{
    x: f32,
    y: f32
}

#[derive(SystemDesc)]
pub struct CharacterSystem {
    is_moving: bool,
    next_frame: usize,
    frame_counter: u8,
    orientation: Option<Orientation>,
    timer_tile: Option<f32>,
    timer_movement: Option<f32>,
}

impl CharacterSystem {
    pub fn new() -> CharacterSystem {
        CharacterSystem {
            is_moving: false,
            next_frame: 0,
            frame_counter: 0,
            orientation: Some(Orientation::DOWN),
            timer_tile: None,
            timer_movement: None,
        }
    }

    fn init_movement(&mut self, orientation: Orientation) {
        self.frame_counter = 16 as u8;
        self.orientation.replace(orientation);
        self.is_moving = true;
        self.timer_movement = Some(TILE_MOVEMENT_DURATION / 16.0);

        let next_frame = match self.orientation {
            Some(Orientation::RIGHT) => 12,
            Some(Orientation::LEFT) => 8,
            Some(Orientation::DOWN) => 0,
            Some(Orientation::UP) => 4,
            None => panic!("Call next frame with no orientation. Stopping.")
        };

        self.next_frame = next_frame;
    }

    fn handle_sprite_update(&mut self, delta_second: f32) {
        if self.frame_counter % 4 == 0 && self.frame_counter < 16{
                self.next_frame += 1;
           
        }else if self.frame_counter == 1{
            self.next_frame -= 3; 
        }
    } 

    fn calculate_smooth_movement(&mut self, delta_second: f32) -> f32 {
        if let Some(mut val) = self.timer_movement {
            val -= delta_second;
            if val <= 0.0 {     
                self.timer_movement.replace(TILE_MOVEMENT_DURATION / 16.0);
                self.frame_counter -= 1;
                2.0
            } else{
                self.timer_tile.replace(val);
                0.0
            }
        }else{
            0.0
        }
    }

    fn calculate_translation(&self, multiplier: f32)  -> Movement{
        match self.orientation {
            Some(Orientation::RIGHT) => Movement {x: multiplier, y: 0.0 },
            Some(Orientation::LEFT) => Movement {x: -1.0 * multiplier, y: 0.0},
            Some(Orientation::DOWN) => Movement {x: 0.0, y:-1.0 * multiplier},
            Some(Orientation::UP) => Movement {x: 0.0, y: multiplier},
            None => panic!("Call next frame with no orientation. Stopping.")
        }
    }
}


impl<'s> System<'s> for CharacterSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MainCharacter>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    ); 

    fn run(&mut self, (mut transforms, characters, input, mut spriterenders, time): Self::SystemData) {
        if self.is_moving && self.frame_counter == 0 {
            self.is_moving = false;
        }

        if !self.is_moving {
            if let Some(orientation) = read_movements_actions(&input) {
                self.init_movement(orientation);
            }
        }

        if self.is_moving {
            let delta_time = time.delta_seconds();
            self.handle_sprite_update(delta_time);
            let smooth_movement_multiplier = self.calculate_smooth_movement(delta_time);
            let movement = self.calculate_translation(smooth_movement_multiplier);
            
            for (_character, transform, spriterender) in (&characters, &mut transforms, &mut spriterenders).join() {
                spriterender.sprite_number = self.next_frame;
                transform.append_translation_xyz(movement.x, movement.y, 0.0);
            }
        }
    }
}

fn read_movements_actions(input: &InputHandler<StringBindings>) -> Option<Orientation> {
    
    let horizontal_movement = 
        ({ if let Some(true) = input.action_is_down("left") { -1 } else { 0 } }) +
        ({ if let Some(true) = input.action_is_down("right") { 1 } else { 0 } });
    let vertical_movement = 
        ({ if let Some(true) = input.action_is_down("up") { -1 } else { 0 } }) +
        ({ if let Some(true) = input.action_is_down("down") { 1 } else { 0 } });

    match (horizontal_movement, vertical_movement) {
        (1, _) => Some(Orientation::RIGHT),
        (-1, _) => Some(Orientation::LEFT),
        (_, 1) => Some(Orientation::DOWN),
        (_, -1) => Some(Orientation::UP),
        (_, _) => None
    }
}
