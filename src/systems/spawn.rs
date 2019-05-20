use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, Cleared, RandomStream};

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        WriteStorage<'a, Coordinates>,
        Write<'a, Cleared>,
        Write<'a, RandomStream>,
        Entities<'a>,
    );

    fn run(&mut self,
           (mut coordinates, mut cleared_flag, mut random_stream, mut entities): Self::SystemData) {
        if cleared_flag.field_cleared {
            let next_layout = get_layout(random_stream.advance());
            let mut next_coordinates = Vec::new();
            for (x_offset, y_offset) in next_layout {
                next_coordinates.push(Coordinates{x: x_offset + SPAWN_POINT.0, y: y_offset + SPAWN_POINT.1})
            }
            for pos in next_coordinates {
                entities
                    .build_entity()
                    .with(pos, &mut coordinates)
                    // TODO select the correct sprite for the block entity
                    .build();
            }
            cleared_flag.field_cleared = false;
        }
    }
}


const SPAWN_POINT: (i32, i32) = (5, 21);

/// Returns a vector describing the positions of all the tetrominoe's blocks
/// The position is relative to be the spawning block ((5, 21) in the standard case)
fn get_layout(piece: Tetrominoes) -> Vec<(i32, i32)> {
    match piece {
        Tetrominoes::I => vec![(-1, 0), (0, 0), (1, 0), (2, 0)],
        Tetrominoes::J => vec![(-1, 1), (-1, 0), (0, 0), (1, 0)],
        Tetrominoes::L => vec![(-1, 0), (0, 0), (1, 0), (1, 1)],
        Tetrominoes::O => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        Tetrominoes::S => vec![(-1, 0), (0, 0), (0, 1), (1, 1)],
        Tetrominoes::T => vec![(-1, 0), (0, 1), (0, 0), (1, 0)],
        Tetrominoes::Z => vec![(-1, 1), (0, 1), (0, 0), (1, 0)],
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