use std::collections::HashMap;
use std::time::Duration;

use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, Resources, System, SystemData, Write, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

use crate::constants::{INPUT_TYPES, KEY_REPEAT_THRESHOLD, KEY_REPEAT_TIME};

pub struct KeyUpdateSystem {
    key_data: HashMap<String, (bool, Duration, Duration)>,
}

impl KeyUpdateSystem {
    pub fn new() -> KeyUpdateSystem {
        KeyUpdateSystem{key_data: HashMap::new()}
    }
}

impl <'a> System<'a> for KeyUpdateSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
        Write<'a, EventChannel<KeyEvent>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (input, mut channel, time): Self::SystemData) {
        let time_delta = time.delta_time();
        // Create a list of all actions that are currently incoming
        let mut actions: Vec<&str> = Vec::new();
        for input_type in &INPUT_TYPES {
            if input.action_is_down(&input_type.to_string())
                .expect("Invalid key input type: Check whether the const array of key definitions and binding_config.ron are identical") {
                actions.push(input_type);
            }
        }

        // Create a Vec which will store which KeyEvents will be written to the output channel
        let mut write_out: Vec<KeyEvent> = Vec::new();

        for input_type in &INPUT_TYPES {
            // Get the information corresponding to the input_type being checked from the HashMap
            let (active, total_time, repeat_time) = self.key_data.get_mut(&input_type.to_string()).unwrap();
            match (actions.contains(input_type), &active) {
                // key is pressed and was pressed down before
                (true, true) => {
//                    println!("{:?}, {:?}", total_time, repeat_time);
                    // Check whether the minimum time for the key to start repeating has been reached
                    if total_time > &mut KEY_REPEAT_THRESHOLD {
                        // Check whether the time controlling the repeat frequency has been reached
                        if repeat_time > &mut KEY_REPEAT_TIME {
                            // Send out a KeyEvent and reset the repeat duration
                            write_out.push(str_to_key_event(input_type));
                            *repeat_time = Duration::from_secs(0);
                        } else {
                            // Increase the repeat duration
                            *repeat_time += time_delta;
                        }
                    } else {
                        // Increase the total duration the key has been held down
                        *total_time += time_delta;
                    }
                },
                // key is pressed but wasn't pressed down before
                (true, false) => {
                    // Send out a KeyEvent and mark the key as having been activated
                    *active = true;
                    write_out.push(str_to_key_event(input_type));
                },
                // key is not pressed but was previously
                (false, true) => {
                    // Reset the values corresponding to the key
                    *active = false;
                    *total_time = Duration::from_secs(0);
                    *repeat_time = Duration::from_millis(1) + KEY_REPEAT_TIME;
                },
                // key neither is nor was pressed
                (false, false) => continue
            }
        }

        channel.iter_write(write_out);
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        // Generate a default value for all input types
        for input in &INPUT_TYPES {
            self.key_data.insert(input.to_string(), (
                false,
                Duration::from_secs(0),
                Duration::from_secs(1) + KEY_REPEAT_TIME
            ));
        }
    }
}


fn str_to_key_event(key_string: &str) -> KeyEvent {
    match key_string {
        "left" => KeyEvent::Left,
        "right" => KeyEvent::Right,
        "rotate"=> KeyEvent::RotateClockwise,
        "descend" => KeyEvent::Descend,
        "drop" => KeyEvent::Drop,
        _ => panic!("invalid key input type received")
    }
}

pub enum KeyEvent {
    Left,
    Right,
    RotateClockwise,
    RotateCounterClockwise,
    Descend,
    Drop,
}