use bevy::prelude::*;

//Resources
#[derive(Resource)]
struct UpdateTimer(Timer);

#[derive(Resource)]
struct TimerCounter {
    i: u32,
}

impl Default for TimerCounter {
    fn default() -> Self {
        TimerCounter { i: 0 }
    }
}

//Components
#[derive(Component)]
enum Direction {
    Up,
    Down,
}

//Systems
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icon.png"),
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
            ..default()
        },
        Direction::Up,
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn timer_print(
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut counter: ResMut<TimerCounter>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Counter: {}", counter.i);
        counter.i += 1;
        if counter.i == 10 {
            counter.i = 0;
        }
    }
}

//Plugins
pub struct CounterPlugin;
impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(TimerCounter { i: 0 })
            .add_systems(Update, timer_print);
    }
}

//Main
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CounterPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

