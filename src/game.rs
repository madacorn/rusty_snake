use crate::GameState;
use bevy::prelude::*;

fn load_game(mut _commands: Commands) {
    println!("Loading game");
}

fn handle_input(mut _commands: Commands) {
    println!("Handle Input");
}
fn move_snake(mut _commands: Commands) {
    println!("Move Snake");
}
fn detect_collision(mut _commands: Commands) {
    println!("Detect Collision");
}
fn spawn_fruits(mut _commands: Commands) {
    println!("Spawn fruits");
}
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), load_game)
            .add_systems(
                Update,
                (handle_input, move_snake, detect_collision, spawn_fruits)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
