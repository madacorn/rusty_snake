use bevy::prelude::*;

mod menu;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, menu::MenuPlugin))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}
