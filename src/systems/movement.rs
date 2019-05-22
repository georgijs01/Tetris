use amethyst::ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{Block, RandomStream, RotationCenter, SpawnTimer};

pub struct MovementSystem;

impl <'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Block>,
        Read<'a, InputHandler<String, String>>
    );

    fn run(&mut self, (mut blocks, input): Self::SystemData) {

    }
}