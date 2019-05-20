use amethyst::core::{Transform, timing::Time};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, SpawnTimer, RandomStream, RotationCenter, GravityTimer, Falling};\\

pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Coordinates>,
        WriteStorage<'a, Falling>,
        Write<'a, RotationCenter>,
        Write<'a, GravityTimer>,
        Write<'a, SpawnTimer>,
    );

    fn run(&mut self, (
        mut coordinates,
        mut falling,
        mut rotation_center,
        mut gravity_timer,
        mut spawn_timer):
    Self::SystemData) {
        gravity_timer.reset();
        // Before moving the active blocks down, the system needs to check whether there is space
        // below. If not, the current blocks will be marked as inactive
        let mut allow_gravity = true;
        'falling: for (falling_coords, _) in (&coordinates, &falling).join() {
            //Check whether the bottom of the play field has been reached
            if falling_coords.x == 0 {
                allow_gravity = false;
                break;
            }
            for (stationary_coords, _) in (&coordinates, !&falling).join() {
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

        } else {

        }
    }
}