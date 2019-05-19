use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::ops::Add;

pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(&self, coords: Coordinates) -> Coordinates {
        Coordinates {x: self.x + coords.x, y: self.y + coords.y}
    }
}

impl Component for Coordinates {
    type Storage = DenseVecStorage<Self>;
}

impl Coordinates {

    pub fn rotate_clockwise(&self, reference: Coordinates) -> Coordinates {
            
    }
}