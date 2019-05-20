use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage, VecStorage};
use rand::Rng;
use core::borrow::{BorrowMut, Borrow};
use crate::systems::spawn::Tetrominos;
use std::time::Duration;


/// Internal coordinate component used by the blocks to mark relative positions on the field
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Component for Coordinates {
    type Storage = DenseVecStorage<Self>;
}



/// A flag that is set on the blocks that belong to the currently active Tetromino
pub struct Gravity {
    pub enabled: bool,
}

impl Gravity {
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Component for Gravity {
    type Storage = VecStorage<Self>;
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity { enabled: true }
    }
}


/// Resource for keeping track of the position of the current rotation center
pub struct RotationCenter {
    pub x: i32,
    pub y: i32,
}

impl Default for RotationCenter {
    fn default() -> Self {
        RotationCenter { x: 0, y: 0 }
    }
}

/// Keeps track of the time since the last clear occurred so that the spawn system knows when to
/// spawn the next entity.
///
/// Is set by the gravity system and reset by the spawn system.
pub struct SpawnTimer {
    time_since_clear: Duration,
    spawn_threshold: Duration,
    active: bool,
}

impl SpawnTimer {
    pub fn add_time(&mut self, time: Duration) {
        if self.active {
            self.time_since_clear += time;
        }
    }

    pub fn should_spawn(&self) -> bool {
        self.time_since_clear > self.spawn_threshold && self.active
    }

    pub fn reset(&mut self) {
        self.time_since_clear = Duration::new(0,0);
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
}

impl Default for SpawnTimer {
    fn default() -> Self {
        // The default SpawnTimer will always trigger a spawn instantly
        Self {
            time_since_clear: Duration::from_secs(2),
            spawn_threshold: Duration::from_secs(1),
            active: true,
        }
    }
}



/// Keeps track of the time since the last time that gravity was applied
/// Implements methods for reducing the time between gravity applications
///
/// Is both set and reset by the gravity system
pub struct GravityTimer {
    timer: Duration,
    threshold: Duration,
}

impl GravityTimer {
    pub fn add_time(&mut self, time: Duration) {
        self.timer += time;
    }

    pub fn should_apply_gravity(&self) -> bool {
        self.timer > self.threshold
    }

    pub fn reset(&mut self) {
        self.timer = Duration::new(0, 0);
    }

    pub fn set_threshold(&mut self, time: Duration) {
        self.threshold = time;
    }
}

impl Default for GravityTimer {
    fn default() -> Self {
        Self {
            timer: Duration::from_millis(0),
            threshold: Duration::from_millis(500),
        }
    }
}


/// Keeps track of the current and upcoming pieces and generates the next pieces as they are needed
pub struct RandomStream {
    pub next_nums: Vec<u8>,
    high: u8
}

impl RandomStream {
    pub fn advance(&mut self) -> Tetrominos {
        let last = self.next_nums.len() - 1;
        for i in 0..last - 1 {
            self.next_nums[i] = self.next_nums[i + 1];
        }
        self.next_nums[last] = rand::thread_rng().gen_range(0, self.high);
        Tetrominos::num_to_tetromino(self.next_nums[0])
    }
}

impl Default for RandomStream {
    fn default() -> Self {
        let low = 0;
        let high = 7;
        let length = 4;
        let mut rng = rand::thread_rng();
        let mut initial_nums = vec![0 as u8; length];
        for _i in 0..length {
            let random_num: u8 = rng.gen_range(low, high);
            initial_nums.push(random_num);
        }
        Self { next_nums: initial_nums, high }
    }
}


pub struct LayoutConfig {
    pub tile_size: i32,
    pub stack_x: u32,
    pub stack_y: u32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {tile_size: 32, stack_x: 0, stack_y: 0}
    }
}