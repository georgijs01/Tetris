use amethyst::ecs::{Entities, Join, Read, ReadExpect, ReadStorage, Resources, System, SystemData, Write, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::shrev::{EventChannel, ReaderId};

use crate::components::{Block, RandomStream, RotationCenter, SpawnTimer};
use crate::constants::ARENA_WIDTH;
use crate::systems::key_update::KeyEvent;

pub struct TranslationSystem {
    channel_reader: Option<ReaderId<KeyEvent>>,
}

impl TranslationSystem {
    pub fn new() -> TranslationSystem {
        TranslationSystem {channel_reader: None}
    }
}

impl <'a> System<'a> for TranslationSystem {
    type SystemData = (
        WriteStorage<'a, Block>,
        Write<'a, RotationCenter>,
        Read<'a, EventChannel<KeyEvent>>,
    );

    fn run(&mut self, (
        mut blocks,
        mut rotation_center,
        channel
    ): Self::SystemData) {
        for event in channel.read(&mut self.channel_reader.as_mut().unwrap()) {
            let translation = match event {
                KeyEvent::Left => -2,
                KeyEvent::Right => 2,
                _ => continue,
            };

            // Check whether the desired movement is allowed
            let mut allowed_movement = true;
            for block in (&blocks).join() {
                if block.falling {
                    //Check whether moving with the desired translation would move the block out of the arena bounds
                    if block.x + translation < 0 || block.x + translation > ARENA_WIDTH - 2 {
                        allowed_movement = false;
                        break;
                    }
                    // Check whether moving with the desired translation would collide with stationary blocks
                    for other_block in (&blocks).join() {
                        if !other_block.falling {
                            if other_block.y == block.y && other_block.x == block.x + translation {
                                allowed_movement = false;
                                break
                            }
                        }
                    }
                }
            }

            // If the movement is allowed, move all falling blocks and the rotation center in the desired direction
            if allowed_movement {
                for block in (&mut blocks).join() {
                    if block.falling {
                        block.x += translation;
                    }
                }
                rotation_center.x += translation;
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.channel_reader = Some(res.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
    }
}