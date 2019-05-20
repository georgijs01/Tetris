use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, Cleared, RandomStream};

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        WriteStorage<'a, Coordinates>,
        Write<'a, Cleared>,
        Write<'a, RandomStream>,
    );

    fn run(&mut self, (mut coordinates, mut cleared_flag, mut random_stream): Self::SystemData) {
        if cleared_flag.field_cleared {
            let next = random_stream.advance();
        }
    }
}

pub enum Tetrominoes {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetrominoes {
    pub fn num_to_tetromino(num: u8) -> Tetrominoes {
        use Tetrominoes::{I, J, L, O, S, T, Z};
        match num {
            0 => I,
            1 => J,
            2 => L,
            3 => O,
            4 => S,
            5 => T,
            6 => Z,
            _ => panic!("Invalid index for a Tetromino (maximum is 6, given was {}", num),
        }
    }
}

