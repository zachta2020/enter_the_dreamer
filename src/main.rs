use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel},};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod components;
mod systems;
mod counter_plugin;
//use crate::counter_plugin::CounterPlugin;
mod console_plugin;
use crate::console_plugin::ConsolePlugin;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 480.;

//Main
fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window{
                title: "Enter the Dreamer Sandbox".into(),
                resolution: (WIDTH, HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                window_level: WindowLevel::AlwaysOnTop,
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()),
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.), 
        LdtkPlugin,
        //RapierDebugRenderPlugin::default(),
        //LogDiagnosticsPlugin::default(),
        //FrameTimeDiagnosticsPlugin::default(),
        ConsolePlugin,
        ))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04))) //changes background color
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::spawn_wall_collision)
        .add_systems(Update, systems::movement)
        .add_systems(Update, systems::spawn_ground_sensor)
        .add_systems(Update, systems::ground_detection)
        .add_systems(Update, systems::update_on_ground)
        .add_systems(Update, systems::refresh_jumps)
        .add_systems(Update, systems::spawn_wall_sensor)
        .add_systems(Update, systems::wall_detection)
        .add_systems(Update, systems::update_on_wall)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .insert_resource(LevelSelection::Index(0))
        .run();
}

