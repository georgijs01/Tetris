use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage, VecStorage};
use rand::Rng;
use std::sync::{Arc, Mutex};
use core::borrow::{BorrowMut, Borrow};
use crate::systems::spawn::Tetrominoes;

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Component for Coordinates {
    type Storage = DenseVecStorage<Self>;
}


pub struct Falling;

impl Component for Falling {
    type Storage = NullStorage<Self>;
}

impl Default for Falling {
    fn default() -> Self {
        Falling
    }
}


#[derive(Default)]
pub struct RotationCenter {
    pub x: i32,
    pub y: i32,
}

pub struct Cleared {
    pub field_cleared: bool,
}

impl Default for Cleared {
    fn default() -> Self {
        Self {field_cleared: true}
    }
}


// Keeps track of the current and upcoming pieces and generates the next pieces as they are needed
pub struct RandomStream {
    pub next_nums: Vec<u8>,
    high: u8
}

impl RandomStream {
    pub fn advance(&mut self) -> Tetrominoes {
        let last = self.next_nums.len() - 1;
        for i in 0..last - 1 {
            self.next_nums[i] = self.next_nums[i + 1];
        }
        self.next_nums[last] = rand::thread_rng().gen_range(0, self.high);
        Tetrominoes::num_to_tetromino(self.next_nums[0])
    }
}

impl Default for RandomStream {
    fn default() -> Self {
        let low = 0;
        let high = 7;
        let length = 4;
        let mut rng = rand::thread_rng();
        let mut initial_nums = vec![0 as u8; length];
        for i in 0..length {
            let random_num: u8 = rng.gen_range(low, high);
            initial_nums.push(random_num);
        }
        Self { next_nums: initial_nums, high }
    }
}