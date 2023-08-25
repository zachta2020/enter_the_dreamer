use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowLevel},};

mod counter_plugin;
use crate::counter_plugin::CounterPlugin;

//Components
#[derive(Component)]
struct Player {

}

enum VerticalDirection {
    Up,
    Down
}

enum HorizontalDirection {
    Left,
    Right
}

#[derive(Component)]
struct Direction {
    vertical: VerticalDirection,
    horizontal: HorizontalDirection
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
        Direction {
            vertical: VerticalDirection::Up,
            horizontal: HorizontalDirection::Right    
        },
    ));
}

 fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match logo.vertical {
            VerticalDirection::Up => transform.translation.y += 150. * time.delta_seconds(),
            VerticalDirection::Down => transform.translation.y -= 150. * time.delta_seconds()
        }
        match logo.horizontal {
            HorizontalDirection::Left => transform.translation.x -= 150. * time.delta_seconds(),
            HorizontalDirection::Right => transform.translation.x += 150. * time.delta_seconds()
        }

        if transform.translation.y > 200. {
            logo.vertical = VerticalDirection::Down;
        } else if transform.translation.y < -200. {
            logo.vertical = VerticalDirection::Up;
        }

        if transform.translation.x > 300. {
            logo.horizontal = HorizontalDirection::Left;
        } else if transform.translation.x < -300. {
            logo.horizontal = HorizontalDirection::Right;
        }
    }
}

//Main
fn main() {
    let (width, height) = (800., 600.);

    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window{
                title: "Enter the Dreamer Sandbox".into(),
                resolution: (width, height).into(),
                present_mode: PresentMode::AutoVsync,
                window_level: WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }), 
        CounterPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

