extern crate amethyst;
extern crate rand;

use amethyst::assets::ProgressCounter;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline,
                         RenderBundle, Stage};
use amethyst::utils::application_dir;

use states::load::LoadingState;

use crate::systems::key_update::KeyEvent;

pub mod components;
pub mod systems;
pub mod states;
pub mod constants;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let config_path = application_dir("resources/display_config.ron")?;
    let display_config = DisplayConfig::load(&config_path);

    let binding_path = application_dir("resources/binding_config.ron")?;

    let render_pipe = Pipeline::build().
        with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(render_pipe, Some(display_config))
                .with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;

    let mut game =
        Application::new("./", LoadingState::new(), game_data)?;

    game.run();

    Ok(())
}
