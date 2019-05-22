use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{Block, GravityTimer, LayoutConfig, RandomStream, RotationCenter, SpawnTimer};

// System which updates the positions of all blocks on the screen based on their current coordinates
// Also adds a transform component to blocks that have not been fully initialized yet
pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        WriteStorage<'a, Block>,
        WriteStorage<'a, Transform>,
        Read<'a, LayoutConfig>,
        Entities<'a>
    );


    fn run(&mut self, (
        mut blocks,
        mut transforms,
        layout_config,
        entities):
    Self::SystemData) {
        for (entity, block) in (&*entities, &mut blocks).join() {
            if !block.initialized {
                let trans = Transform::default();
                transforms.insert(entity, trans);
                block.initialized = true;
            }
        }

        for (block, transform) in (&blocks, &mut transforms).join() {
            let xy = xy_from_coordinates(&layout_config, block.x, block.y);
            transform.set_translation_xyz(xy.0, xy.1, 0.);
        }
    }
}

fn xy_from_coordinates(layout: &LayoutConfig, x: i32, y: i32) -> (f32, f32) {
    (
        (layout.stack_x + layout.tile_size * (x + 1) / 2) as f32,
        (layout.stack_y + layout.tile_size * (y + 1) / 2) as f32,
    )
}


