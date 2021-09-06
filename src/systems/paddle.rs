use amethyst::{
    core::{Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, _transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(movement_amount) = movement {
                if movement_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving {}", side_name, movement_amount);
                }
            }
        }
    }
}

