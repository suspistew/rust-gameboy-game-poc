use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

use crate::entities::MainCharacter;
use crate::game::TILE_SIZE;

const TILE_MOVEMENT_DURATION_IN_LOOP: f32 = 16.0;

enum Orientation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Movement {
    x: f32,
    y: f32,
}

#[derive(SystemDesc)]
pub struct CharacterSystem {
    is_moving: bool,
    next_frame: usize,
    frame_counter: u8,
    orientation: Option<Orientation>,
    timer_movement: Option<f32>,
}

impl CharacterSystem {
    pub fn new() -> CharacterSystem {
        CharacterSystem {
            is_moving: false,
            next_frame: 0,
            frame_counter: 0,
            orientation: Some(Orientation::DOWN),
            timer_movement: None,
        }
    }

    fn init_movement(&mut self, orientation: Orientation) {
        self.frame_counter = TILE_MOVEMENT_DURATION_IN_LOOP as u8;
        self.orientation.replace(orientation);
        self.is_moving = true;
        self.timer_movement = Some(TILE_MOVEMENT_DURATION_IN_LOOP);

        let next_frame = match self.orientation {
            Some(Orientation::RIGHT) => 12,
            Some(Orientation::LEFT) => 8,
            Some(Orientation::DOWN) => 0,
            Some(Orientation::UP) => 4,
            None => panic!("Call next frame with no orientation. Stopping."),
        };

        self.next_frame = next_frame;
    }

    fn handle_sprite_update(&mut self) {
        if self.frame_counter % ((TILE_MOVEMENT_DURATION_IN_LOOP / 4.0) as u8) == 0
            && self.frame_counter < TILE_MOVEMENT_DURATION_IN_LOOP as u8
        {
            self.next_frame += 1;
        } else if self.frame_counter == 1 {
            self.next_frame -= 3;
        }
    }

    fn calculate_smooth_movement(&mut self) -> f32 {
        if let Some(mut val) = self.timer_movement {
            val -= 1.0;
            if val <= 0.0 {
                self.timer_movement.replace(TILE_MOVEMENT_DURATION_IN_LOOP);
                self.is_moving = false;
                self.frame_counter -= 1;
                TILE_SIZE / TILE_MOVEMENT_DURATION_IN_LOOP
            } else {
                self.frame_counter -= 1;
                self.timer_movement.replace(val);
                TILE_SIZE / TILE_MOVEMENT_DURATION_IN_LOOP
            }
        } else {
            0.0
        }
    }

    fn calculate_translation(&self, multiplier: f32) -> Movement {
        match self.orientation {
            Some(Orientation::RIGHT) => Movement {
                x: multiplier,
                y: 0.0,
            },
            Some(Orientation::LEFT) => Movement {
                x: -1.0 * multiplier,
                y: 0.0,
            },
            Some(Orientation::DOWN) => Movement {
                x: 0.0,
                y: -1.0 * multiplier,
            },
            Some(Orientation::UP) => Movement {
                x: 0.0,
                y: multiplier,
            },
            None => panic!("Call next frame with no orientation. Stopping."),
        }
    }
}

impl<'s> System<'s> for CharacterSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MainCharacter>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut transforms, characters, input, mut spriterenders): Self::SystemData) {
        if self.is_moving && self.frame_counter == 0 {
            self.is_moving = false;
        }

        if !self.is_moving {
            if let Some(orientation) = read_movements_actions(&input) {
                self.init_movement(orientation);
            }
        }

        if self.is_moving {
            self.handle_sprite_update();
            let smooth_movement_multiplier = self.calculate_smooth_movement();
            let movement = self.calculate_translation(smooth_movement_multiplier);

            for (_character, transform, spriterender) in
                (&characters, &mut transforms, &mut spriterenders).join()
            {
                spriterender.sprite_number = self.next_frame;
                transform.append_translation_xyz(movement.x, movement.y, 0.0);
            }
        }
    }
}

fn read_movements_actions(input: &InputHandler<StringBindings>) -> Option<Orientation> {
    let horizontal_movement = ({
        if let Some(true) = input.action_is_down("left") {
            -1
        } else {
            0
        }
    }) + ({
        if let Some(true) = input.action_is_down("right") {
            1
        } else {
            0
        }
    });
    let vertical_movement = ({
        if let Some(true) = input.action_is_down("up") {
            -1
        } else {
            0
        }
    }) + ({
        if let Some(true) = input.action_is_down("down") {
            1
        } else {
            0
        }
    });

    match (horizontal_movement, vertical_movement) {
        (1, _) => Some(Orientation::RIGHT),
        (-1, _) => Some(Orientation::LEFT),
        (_, 1) => Some(Orientation::DOWN),
        (_, -1) => Some(Orientation::UP),
        (_, _) => None,
    }
}
