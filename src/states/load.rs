use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
//use crate::components::SpriteResource;

pub const VIEW_WIDTH: f32 = 100.;
pub const VIEW_HEIGHT: f32 = 100.;


pub struct LoadingState {
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        init_camera(data.world);
        let sprite_sheet_handle = load_sprite_sheet(data.world);
        data.world.add_resource(sprite_sheet_handle.clone());

        let mut transform = Transform::default();
        transform.set_translation_xyz(100., 100., 2.);
        let sprite_render = SpriteRender {sprite_sheet: sprite_sheet_handle.clone(), sprite_number: 0};
        data.world
            .create_entity()
            .with(transform)
            .with(sprite_render)
            .build();
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

fn load_sprite_sheet(world: &World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "./resources/texture/tetris_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "./resources/texture/tetris_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
//    world.add_resource(SpriteResource { handle: Some(texture_handle) });
}