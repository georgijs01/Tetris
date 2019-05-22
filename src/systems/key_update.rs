use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

pub struct KeyUpdateSystem;

impl <'a> System<'a> for KeyUpdateSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
        Write<'a, EventChannel<KeyEvent>>,
    );

    fn run(&mut self, (input, mut channel): Self::SystemData) {

    }
}


pub enum KeyEvent {
    LEFT,
    RIGHT,
    ROTATE,
    DESCEND,
    DROP,
}