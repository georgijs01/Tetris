extern crate amethyst;
extern crate rand;

pub mod components;
pub mod systems;
pub mod states;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use amethyst::utils::application_dir;

use states::gameplay::GameplayState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let config_path = application_dir("resources/display_config.ron")?;
    let display_config = DisplayConfig::load(&config_path);


    let render_pipe = Pipeline::build().
        with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(render_pipe, Some(display_config))
                .with_sprite_sheet_processor()
        )?;

    let mut game =
        Application::new("./", states::load::LoadingState{}, game_data)?;

    game.run();

    Ok(())
}
