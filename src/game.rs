use crate::GameState;
use bevy::prelude::*;

fn load_game(mut _commands: Commands) {
    println!("Loading game");
}
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), load_game);
    }
}
