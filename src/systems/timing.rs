use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

use crate::components::{Block, GravityTimer, RandomStream, RotationCenter, SpawnTimer};

/// A system to update the various timers that the game relies upon in one centralised system
pub struct TimingSystem;

impl<'a> System<'a> for TimingSystem {
    type SystemData = (
        Write<'a, GravityTimer>,
        Write<'a, SpawnTimer>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut gravity_timer,
        mut spawn_timer,
        time
    ): Self::SystemData) {
        let time_delta = time.delta_time();
        gravity_timer.add_time(time_delta);
        spawn_timer.add_time(time_delta);
    }
}