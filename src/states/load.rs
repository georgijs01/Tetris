use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const VIEW_WIDTH: f32 = 100.;
pub const VIEW_HEIGHT: f32 = 100.;


pub struct LoadingState {
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        init_camera(data.world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
        0.0, VIEW_WIDTH, 0.0, VIEW_HEIGHT)))
        .with(transform)
        .build();
}