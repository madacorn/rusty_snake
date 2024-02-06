use crate::GameState;
use bevy::prelude::*;

#[derive(Resource, Default)]
enum SnakeDirection {
    #[default]
    Up,
    Left,
    Down,
    Right,
}

fn load_game(mut _commands: Commands) {
    println!("Loading game");
}

fn handle_input(keys: Res<Input<KeyCode>>, mut direccion: ResMut<SnakeDirection>) {
    if keys.just_pressed(KeyCode::W) {
        match *direccion {
            SnakeDirection::Left | SnakeDirection::Right => *direccion = SnakeDirection::Up,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::A) {
        match *direccion {
            SnakeDirection::Up | SnakeDirection::Down => *direccion = SnakeDirection::Left,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::S) {
        match *direccion {
            SnakeDirection::Left | SnakeDirection::Right => *direccion = SnakeDirection::Down,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::D) {
        match *direccion {
            SnakeDirection::Up | SnakeDirection::Down => *direccion = SnakeDirection::Right,
            _ => (),
        }
    }
}
fn move_snake(_time: Res<Time>) {
    //println!("Move Snake");
}
fn detect_collision(mut _commands: Commands) {
    //println!("Detect Collision");
}
fn spawn_fruits(mut _commands: Commands) {
    //println!("Spawn fruits");
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
            )
            .init_resource::<SnakeDirection>();
    }
}
