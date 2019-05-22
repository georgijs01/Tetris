use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::input::InputHandler;

pub struct KeyUpdateSystem;

impl <'a> System<'a> for KeyUpdateSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>
    );

    fn run(&mut self, data: Self::SystemData) {
        unimplemented!()

    // TODO use the event system
    }
}


pub enum KeyEvent {
    LEFT,
    RIGHT,
    ROTATE,
    DESCEND,
    DROP,
}