use bevy::prelude::*;
use std::io::*;
use crate::components::*;

pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
    app.insert_resource(PlayerConfig::default())
        .add_systems(Update, console)
        .add_systems(Update, update_player_config)
        .add_systems(Update, show_player_config);
    }
}

//resources
#[derive(Resource)]
struct PlayerConfig{
    max_jump_count: i32,
    walk_speed: f32,
    jump_speed: f32,
    running_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
     PlayerConfig {
            max_jump_count: DEFAULT_JUMP_COUNT,
            walk_speed: DEFAULT_HORIZONTAL_WALK,
            jump_speed: DEFAULT_VERTICAL_JUMP,
            running_speed: DEFAULT_HORIZONTAL_RUN,
        }
    }
}

//systems
fn console(
    input: Res<Input<KeyCode>>,
    mut player_config: ResMut<PlayerConfig>){
    if input.just_pressed(KeyCode::Equals) {
        loop {
            println!("\nCURRENT PLAYER CONFIG");
            println!("1. Walk Speed - {}", player_config.walk_speed);
            println!("2. Run Speed - {}", player_config.running_speed);
            println!("3. Jump Speed - {}", player_config.jump_speed);
            println!("4. Max Jump Count - {}", player_config.max_jump_count);
            println!("0. Exit\n");

            let mut line = String::new();
            let _ = stdin().read_line(&mut line);
            let option: i32 = line.trim().parse().expect("ERROR: INVALID OPTION\n");
            println!("");

            if option == 0 {
                println!("Exiting Console...");
                break;
            } else if option == 1 {
                println!("Enter new Walk Speed ");
                line.clear();
                _ = stdin().read_line(&mut line);
                player_config.walk_speed = line.trim().parse().expect("ERROR: INVALID NUMBER");
            } else if option == 2 {
                println!("Enter new Run Speed: ");
                line.clear();
                _ = stdin().read_line(&mut line);
                player_config.running_speed = line.trim().parse().expect("ERROR: INVALID NUMBER");
            } else if option == 3 {
                println!("Enter new Jump Speed: ");
                line.clear();
                _ = stdin().read_line(&mut line);
                player_config.jump_speed = line.trim().parse().expect("ERROR: INVALID NUMBER");
            } else if option == 4 {
                println!("Enter new Max Jump Count: ");
                line.clear();
                _ = stdin().read_line(&mut line);
                player_config.max_jump_count = line.trim().parse().expect("ERROR: INVALID NUMBER");
            } else {
                println!("ERROR: INVALID COMMAND");
            }
        }
    }
}

fn update_player_config(
    player_config: Res<PlayerConfig>,
    mut query: Query<&mut MovementConfig, With<Player>>
) {
    for mut player in &mut query {
        player.walk_speed = player_config.walk_speed;
        player.running_speed = player_config.running_speed;
        player.jump_speed = player_config.jump_speed;
        player.max_jump_count = player_config.max_jump_count;
    }
}

fn show_player_config(
    input: Res<Input<KeyCode>>,
    player_config: Res<PlayerConfig>
) {
    if input.just_pressed(KeyCode::Minus){
        println!("\nCURRENT PLAYER CONFIG");
        println!("Walk Speed - {}", player_config.walk_speed);
        println!("Run Speed - {}", player_config.running_speed);
        println!("Jump Speed - {}", player_config.jump_speed);
        println!("Max Jump Count - {}", player_config.max_jump_count);
    }
}