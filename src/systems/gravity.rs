use amethyst::ecs::{Join, System, SystemData, Write, WriteStorage};
use amethyst::shrev::ReaderId;

use crate::components::{Block, GravityTimer, RotationCenter, SpawnTimer};

pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Block>,
        Write<'a, GravityTimer>,
        Write<'a, SpawnTimer>,
        Write<'a, RotationCenter>,
    );

    //noinspection ALL
    fn run(&mut self, (
        mut blocks,
        mut gravity_timer,
        mut spawn_timer,
        mut rotation_center
    ): Self::SystemData) {
        // Only apply Gravity if the time threshold has been reached
        if gravity_timer.should_apply_gravity() {
            gravity_timer.reset();
            // Before moving the active blocks down, the system needs to check whether there is space
            // below. If not, the current blocks will be marked as inactive
            let mut allow_gravity = true;
            'falling: for block in (&blocks).join() {
                if !block.falling {
                    continue;
                }
                //Check whether the bottom of the play field has been reached
                if block.y == 0 {
                    allow_gravity = false;
                    break;
                }
                for other_block in (&blocks).join() {
                    // Check whether the block is directly beneath the falling block and disabled
                    if !other_block.falling && block.x == other_block.x
                        // NOTE: -2 is a shift ONE block down in coordinate space
                        && block.y - 2 == other_block.y {
                        allow_gravity = false;
                        break 'falling;
                    }
                }
            }

            if allow_gravity {
                // Move all falling blocks down by one tile
                for block in (&mut blocks).join() {
                    if block.falling {
                        block.y -= 2;
                    }
                }
                rotation_center.y -= 2;
            } else {
                // Lock all falling blocks in place
                for block in (&mut blocks).join() {
                    block.falling = false;
                }
                // Activate the spawn timer so that a new piece will appear
                spawn_timer.activate();
            }
        }
    }
}