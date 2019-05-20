use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, SpawnTimer, RandomStream, RotationCenter, Falling};

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        WriteStorage<'a, Coordinates>,
        WriteStorage<'a, Falling>,
        Write<'a, SpawnTimer>,
        Write<'a, RandomStream>,
        Write<'a, RotationCenter>,
        Entities<'a>,
    );

    fn run(&mut self, (
        mut coordinates,
        mut falling,
        mut spawn_timer,
        mut random_stream,
        mut rotation_center,
        mut entities):
    Self::SystemData) {
        if spawn_timer.should_spawn() {
            let next_piece = random_stream.advance();

            // Set the rotation center of the new piece
            let rotation_center_offset = get_rotation_center(&next_piece);
            rotation_center.x = SPAWN_POINT.0 + rotation_center_offset.0;
            rotation_center.y = SPAWN_POINT.1 + rotation_center_offset.1;

            // Get the coordinates of the new blocks
            let mut next_coordinates = Vec::new();
            let next_layout = get_layout(&next_piece);
            for (x_offset, y_offset) in next_layout {
                next_coordinates
                    .push(Coordinates{
                        x: x_offset * 2 + SPAWN_POINT.0,
                        y: y_offset * 2 + SPAWN_POINT.1,
                    })
            }

            // Add the new blocks to the world
            for pos in next_coordinates {
                entities
                    .build_entity()
                    .with(pos, &mut coordinates)
                    .with(Default::default(), &mut falling)
                    // TODO select the correct sprite for the block entity
                    .build();
            }

            spawn_timer.reset();
        }
    }
}

// The spawn point on the grid
const SPAWN_POINT: (i32, i32) = (9, 41);

/// Returns a vector describing the positions of all the tetrominoe's blocks
/// The position is relative to be the spawning block ((5, 21) in the standard case)
fn get_layout(piece: &Tetrominos) -> Vec<(i32, i32)> {
    match piece {
        Tetrominos::I => vec![(-1, 0), (0, 0), (1, 0), (2, 0)],
        Tetrominos::J => vec![(-1, 1), (-1, 0), (0, 0), (1, 0)],
        Tetrominos::L => vec![(-1, 0), (0, 0), (1, 0), (1, 1)],
        Tetrominos::O => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        Tetrominos::S => vec![(-1, 0), (0, 0), (0, 1), (1, 1)],
        Tetrominos::T => vec![(-1, 0), (0, 1), (0, 0), (1, 0)],
        Tetrominos::Z => vec![(-1, 1), (0, 1), (0, 0), (1, 0)],
    }
}

/// Returns the correct rotation center relative to the Spawn point
fn get_rotation_center(piece: &Tetrominos) -> (i32, i32) {
    match piece {
        Tetrominos::I => (1, -1),
        Tetrominos::O => (1, 1),
        _ => (0, 0),
    }
}

pub enum Tetrominos {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetrominos {
    pub fn num_to_tetromino(num: u8) -> Tetrominos {
        use Tetrominos::{I, J, L, O, S, T, Z};
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