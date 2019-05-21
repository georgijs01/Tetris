use amethyst::core::{Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities};
use amethyst::input::InputHandler;
use crate::components::{Coordinates, SpawnTimer, RandomStream, RotationCenter, GravityTimer, LayoutConfig};


// System which updates the positions of all blocks on the screen based on their current coordinates
pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        ReadStorage<'a, Coordinates>,
        WriteStorage<'a, Transform>,
        Read<'a, LayoutConfig>,
    );

    //noinspection ALL
    fn run(&mut self, (coordinates, mut transforms, layout_config): Self::SystemData) {
        for (coordinate, transform) in (&coordinates, &mut transforms).join() {
            let x = layout_config.stack_x + layout_config.tile_size * (coordinate.x + 1) / 2;
            let y = layout_config.stack_y + layout_config.tile_size * (coordinate.y + 1) / 2;
            transform.set_translation_xyz(x as f32, y as f32, 2.);
        }
    }
}


