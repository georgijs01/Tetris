use amethyst::ecs::{Entities, Join, Read, ReadExpect, ReadStorage, Resources, System, SystemData, Write, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::shrev::{EventChannel, ReaderId};

use crate::components::{Block, RandomStream, RotationCenter, SpawnTimer};
use crate::constants::{ARENA_WIDTH, WALL_KICKS};
use crate::systems::key_update::KeyEvent;
use crate::systems::spawn::Tetrominos;

pub struct RotationSystem {
    channel_reader: Option<ReaderId<KeyEvent>>,
}

impl RotationSystem {
    pub fn new() -> RotationSystem {
        RotationSystem {channel_reader: None}
    }
}

impl <'a> System<'a> for RotationSystem {
    type SystemData = (
        WriteStorage<'a, Block>,
        Read<'a, RotationCenter>,
        Read<'a, EventChannel<KeyEvent>>,
    );

    fn run(&mut self, (
        mut blocks,
        rotation_center,
        channel
    ): Self::SystemData) {
        for event in channel.read(&mut self.channel_reader.as_mut().unwrap()) {
            let dir_clockwise = match event {
                KeyEvent::RotateClockwise => true,
                KeyEvent::RotateCounterClockwise => false,
                _ => continue
            };

            let mut rotated_coords: Vec<(i32, i32)> = Vec::new();
            let mut piece = &Tetrominos::I;
            let mut rotation_state = 0;
            for block in (&blocks).join() {
                if block.falling {
                    piece = &block.piece;
                    rotation_state = block.rotation;
                    rotated_coords.push(get_rotated(
                        block.x,
                        block.y,
                        rotation_center.x,
                        rotation_center.y,
                        dir_clockwise
                    ))
                }
            }

            println!("{:?}", rotated_coords);

            let mut possible_kick = -1;
            let wall_kicks = get_wall_kick_data(piece, rotation_state, dir_clockwise);
            'kicks: for (i, (dx, dy)) in wall_kicks.iter().enumerate() {

                let mut rotation_allowed = true;

                'coords: for coord in &rotated_coords {
                    let (x, y) = (coord.0 + dx * 2, coord.1 + dy * 2);
                    if x < 0 || x > ARENA_WIDTH - 2 {
                        rotation_allowed = false;
                        break 'coords;
                    }

                    for block in (&blocks).join() {
                        if !block.falling {
                            if block.x == x && block.y == y {
                                rotation_allowed = false;
                                break 'coords;
                            }
                        }
                    }
                }
                if rotation_allowed {
                    possible_kick = i as i32;
                    break 'kicks;
                }
            }

            println!("{}", possible_kick);

            if !possible_kick < 0 {
                let (dx, dy) = wall_kicks[possible_kick as usize];
                for block in (&mut blocks).join() {
                    if block.falling {
                        let (bx, by) = get_rotated(block.x, block.y, rotation_center.x, rotation_center.y, dir_clockwise);
                        let (x, y)= (bx + dx * 2, by + dy * 2);
                        block.x = x;
                        block.y = y;
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.channel_reader = Some(res.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
    }
}

fn get_wall_kick_data(piece: &Tetrominos, current_rotation: i32, dir_clockwise: bool) -> &'static [(i32, i32); 5] {
    let i = if let piece = Tetrominos::I { 1 } else { 0 };
    let j = current_rotation + if dir_clockwise { 0 } else { 1 };
    &WALL_KICKS[i][j as usize]
}


/// Rotates a point p around the rotation point r, either clockwise or counterclockwise by 90 deg
fn get_rotated(px: i32, py: i32, rx: i32, ry:i32, dir_clockwise: bool) -> (i32, i32) {
    let rel_x = px - rx;
    let rel_y = py - ry;

    let coefficient = if dir_clockwise { 1 } else { -1 };

    let rel_rot_x = rel_y * coefficient;
    let rel_rot_y = -1 * rel_x * coefficient;

    let rot_x = rel_rot_x + rx;
    let rot_y = rel_rot_y + ry;


    (rot_x, rot_y)
}