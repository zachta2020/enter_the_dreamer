use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

pub const DEFAULT_JUMP_COUNT: i32 = 1;
pub const DEFAULT_HORIZONTAL_WALK: f32 = 200.;
pub const DEFAULT_VERTICAL_JUMP: f32 = 500.;
pub const DEFAULT_HORIZONTAL_RUN: f32 = 300.;

#[derive(Component)]
pub struct PrimaryCamera;

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
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    pub movement_config: MovementConfig,

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

#[derive(Clone, Component)]
pub struct MovementConfig {
    pub jump_count: i32,
    pub max_jump_count: i32,
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub running_speed: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
     MovementConfig {
            jump_count: DEFAULT_JUMP_COUNT,
            max_jump_count: DEFAULT_JUMP_COUNT,
            walk_speed: DEFAULT_HORIZONTAL_WALK,
            jump_speed: DEFAULT_VERTICAL_JUMP,
            running_speed: DEFAULT_HORIZONTAL_RUN,
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