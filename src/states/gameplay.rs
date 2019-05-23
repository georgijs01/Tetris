use amethyst::{GameData, SimpleState, SimpleTrans, State, StateData, Trans};
use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder};

use crate::systems::gravity::GravitySystem;
use crate::systems::key_update::KeyUpdateSystem;
use crate::systems::pos_update::PositionUpdateSystem;
use crate::systems::rotation::RotationSystem;
use crate::systems::spawn::SpawnSystem;
use crate::systems::timing::TimingSystem;
use crate::systems::translation::TranslationSystem;

pub struct GameplayState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>
}

impl<'a, 'b> GameplayState<'a, 'b> {
    pub fn new() -> GameplayState<'a, 'b> {
        GameplayState {dispatcher: None}
    }
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(TimingSystem, "timing", &[]);
        dispatcher_builder.add(GravitySystem, "gravity", &["timing"]);
        dispatcher_builder.add(SpawnSystem, "spawn", &["timing"]);
        dispatcher_builder.add(PositionUpdateSystem, "render_update", &[]);
        dispatcher_builder.add(KeyUpdateSystem::new(), "key_update", &[]);
        dispatcher_builder.add(TranslationSystem::new(), "translation", &["key_update"]);
        dispatcher_builder.add(RotationSystem::new(), "rotation", &["key_update"]);

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut data.world.res);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world.res);
        }
        Trans::None
    }


}