use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel},};
use bevy_rapier2d::prelude::*;

mod counter_plugin;
use crate::counter_plugin::CounterPlugin;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 400.;
const THICKNESS: f32 = 20.;

const PLAYER_VELOCITY_X: f32 = 200.0;
const PLAYER_VELOCITY_Y_UP: f32 = 450.0;
const PLAYER_VELOCITY_Y_DOWN: f32 = 550.0;

const MAX_JUMP_HEIGHT: f32 = 100.0;

//Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Jump (f32);

//Bundles
#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(x: f32, y: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

//Systems
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    //player
    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x: 0.0, y: -100., z: 0.0 },
                scale: Vec3::new(20., 20., 1.),
                ..default()
            },
            ..default()
        })
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::cuboid(0.5, 0.5))
            .insert(KinematicCharacterController::default())
            .insert(Player);

    //floor
    commands.spawn(PlatformBundle::new(0.0, HEIGHT / -2.0 + THICKNESS / 2.0 ,Vec3::new(WIDTH, THICKNESS, 1.0)));

    //ceiling
    commands.spawn(PlatformBundle::new(0.0, HEIGHT / 2.0 - THICKNESS / 2.0, Vec3::new(WIDTH, THICKNESS, 1.0)));

    //walls
    commands.spawn(PlatformBundle::new(WIDTH / -2.0 + THICKNESS / 2.0, 0.0, Vec3::new(THICKNESS, HEIGHT - THICKNESS * 2.0, 1.0)));
    commands.spawn(PlatformBundle::new(WIDTH / 2.0 - THICKNESS / 2.0, 0.0, Vec3::new(THICKNESS, HEIGHT - THICKNESS * 2.0, 1.0)));
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::Right) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::Left) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::Space) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y_UP;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    let movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y_DOWN * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

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
        }),
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.), 
        //RapierDebugRenderPlugin::default(),
        CounterPlugin))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04))) //changes background color
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, jump, rise, fall))
        .run();
}

