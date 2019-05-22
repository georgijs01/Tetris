use amethyst::assets::{AssetStorage, Loader, ProgressCounter};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::shrev::EventChannel;

use crate::states::gameplay::GameplayState;
use crate::systems::key_update::KeyEvent;
use crate::systems::timing::UpdateEvent;

//use crate::components::SpriteResource;

pub const VIEW_WIDTH: f32 = 320.;
pub const VIEW_HEIGHT: f32 = 640.;


pub struct LoadingState {
    pub progress_counter: ProgressCounter,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        init_camera(data.world);

        let texture_handle = {
            let loader = data.world.read_resource::<Loader>();
            loader.load(
                "resources/texture/tetris_spritesheet.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Texture>>(),
            )
        };

        let sprite_sheet_handle = {
            let loader = data.world.read_resource::<Loader>();
            loader.load(
                "resources/texture/tetris_spritesheet.ron", // Here we load the associated ron file
                SpriteSheetFormat,
                texture_handle, // We pass it the texture we want it to use
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
            )
        };

        // add SpriteSheetHandle as resource
        data.world.add_resource(sprite_sheet_handle.clone());

        // initialize event channels
        data.world.add_resource(EventChannel::<KeyEvent>::new());
        data.world.add_resource(EventChannel::<UpdateEvent>::new());
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GameplayState{dispatcher: None}))
        } else {
            Trans::None
        }
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