use amethyst::{GameData, SimpleState, SimpleTrans, State, StateData, Trans};
use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder};

use crate::systems::gravity::GravitySystem;
use crate::systems::pos_update::PositionUpdateSystem;
use crate::systems::spawn::SpawnSystem;
use crate::systems::timing::TimingSystem;

pub struct GameplayState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(TimingSystem, "timing", &[]);
        dispatcher_builder.add(GravitySystem, "gravity", &["timing"]);
        dispatcher_builder.add(SpawnSystem, "spawn", &["timing"]);
        dispatcher_builder.add(PositionUpdateSystem, "render_update", &[]);

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