use amethyst::core::{Transform, timing::Time};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, SpawnTimer, RandomStream, RotationCenter, GravityTimer, Gravity};

pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Coordinates>,
        WriteStorage<'a, Gravity>,
        Write<'a, RotationCenter>,
        Write<'a, GravityTimer>,
        Write<'a, SpawnTimer>,
        Entities<'a>,
    );

    //noinspection ALL
    fn run(&mut self, (
        mut coordinates,
        mut gravity_enabled,
        mut rotation_center,
        mut gravity_timer,
        mut spawn_timer,
        mut entities):
    Self::SystemData) {
        // Do nothing if the timer hasn't reached the threshold
        if !gravity_timer.should_apply_gravity() {
            return;
        }

        gravity_timer.reset();
        // Before moving the active blocks down, the system needs to check whether there is space
        // below. If not, the current blocks will be marked as inactive
        let mut allow_gravity = true;
        'falling: for (falling_coords, falling_outer) in (&coordinates, &gravity_enabled).join() {
            //Check whether the bottom of the play field has been reached
            if !falling_outer.enabled {
                continue;
            }
            if falling_coords.y == 0 {
                allow_gravity = false;
                break;
            }
            for (stationary_coords, falling_inner) in (&coordinates, &gravity_enabled).join() {
                if falling_inner.enabled {
                    continue;
                }
                // Check whether the block is directly beneath the falling block
                if falling_coords.x == stationary_coords.x
                    // NOTE: -2 is a shift ONE block down in coordinate space
                    && falling_coords.y - 2 == stationary_coords.y {
                    allow_gravity = false;
                    break 'falling;
                }
            }
        }

        if allow_gravity {
            // Move all falling blocks down by one tile
            for (falling_coords, _) in (&mut coordinates, &gravity_enabled).join() {
                let new_y = falling_coords.y - 2;
                falling_coords.y = new_y;
            }
        } else {
            // Lock all falling blocks in place
            for falling_flag in (&mut gravity_enabled).join() {
                falling_flag.disable();
            }
            // Activate the spawn timer so that a new piece will appear
            spawn_timer.activate();
        }
    }
}