use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;


pub const DEFAULT_HORIZONTAL_WALK: f32 = 10000.;
pub const DEFAULT_HORIZONTAL_RUN: f32 = 15000.;
pub const DEFAULT_HORIZONTAL_DASH: f32 = 30000.;

pub const DEFAULT_JUMP_COUNT: i32 = 1;
pub const DEFAULT_VERTICAL_JUMP: f32 = 30000.;

#[derive(Component, Default)]
pub struct PrimaryCamera;

/* #[derive(Component, Default)]
pub struct CameraCoords {
    pub x: f32,
    pub y: f32,
} */

#[derive(Bundle, Default)]
pub struct PrimaryCameraBundle {
    pub camera_2d: Camera2dBundle,
    pub primary_camera: PrimaryCamera
}



#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub ccd: Ccd,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(12.0, 16.0),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ccd: Ccd::enabled(),
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub movement_bundle: MovementBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
}
#[derive(Clone, Bundle, Default)]
pub struct MovementBundle {
    pub horizontal_mover: HorizontalMover,
    pub vertical_mover: VerticalMover,
}

#[derive(Clone, Component)]
pub struct HorizontalMover {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub current_speed: f32,

    pub can_dash: bool,
    pub is_dashing: bool,
    pub dash_power: f32,
    pub dashing_timer: Timer,
    pub dash_cooldown_timer: Timer,

    pub predash_gravity: f32,

    pub facing_direction: PlayerDirection,
}

impl Default for HorizontalMover {
    fn default() -> Self {
        HorizontalMover {
            walk_speed: DEFAULT_HORIZONTAL_WALK,
            run_speed: DEFAULT_HORIZONTAL_RUN,
            current_speed: 0.0,

            can_dash: true,
            is_dashing: false,
            dash_power: DEFAULT_HORIZONTAL_DASH,
            dashing_timer: Timer::from_seconds(0.2, TimerMode::Once),
            dash_cooldown_timer: Timer::from_seconds(1.0, TimerMode::Once),

            predash_gravity: 0.0,

            facing_direction: PlayerDirection::Right,
        }
    }
}

#[derive(Clone, Component)]
pub struct VerticalMover {
    pub jump_speed: f32,
    pub jump_count: i32,
    pub max_jump_count: i32,
}

impl Default for VerticalMover {
    fn default() -> Self {
        VerticalMover { 
            jump_speed: DEFAULT_VERTICAL_JUMP, 
            jump_count: DEFAULT_JUMP_COUNT,
            max_jump_count: DEFAULT_JUMP_COUNT,
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Clone, Default, Component)]
pub struct WallDetection {
    pub on_wall: bool,
}

#[derive(Component)]
pub struct WallSensor {
    pub wall_detection_entity: Entity,
    pub intersecting_wall_entities: HashSet<Entity>,
}