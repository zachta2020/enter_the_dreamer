use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::time::Duration;
use std::collections::HashSet;


/* pub const DEFAULT_HORIZONTAL_WALK: f32 = 10000.;
pub const DEFAULT_HORIZONTAL_RUN: f32 = 15000.;
pub const DEFAULT_HORIZONTAL_DASH: f32 = 30000.; */

/* pub const DEFAULT_JUMP_COUNT: i32 = 1;
pub const DEFAULT_VERTICAL_JUMP: f32 = 30000.; */

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

#[derive(Clone, Debug, PartialEq)]
pub enum FacingDirection {
    Left,
    Right,
}

impl FacingDirection {
    pub fn get_opposite(&self) -> FacingDirection {
        if *self == FacingDirection::Left {
            FacingDirection::Right
        } else {
            FacingDirection::Left
        }
    }
}

#[derive(Clone, Bundle, Default)]
pub struct MovementBundle {
    pub horizontal_mover: HorizontalMover,
    pub vertical_mover: VerticalMover,
    pub dasher: Dasher,
}

#[derive(Clone, Component)]
pub struct HorizontalMover {
    pub is_horizontal_moving: bool,

    pub walk_speed: f32,
    pub walk_acc: f32,
    pub walk_dec: f32,
    pub walk_turn: f32,

    pub run_speed: f32,
    pub run_acc: f32,
    pub run_dec: f32,
    pub run_turn: f32,

    pub air_speed: f32,
    pub air_acc: f32,
    pub air_dec: f32,
    pub air_turn: f32,

    pub current_speed: f32,



    pub facing_direction: FacingDirection,
}
#[derive(Clone, Component)]
pub struct Dasher {
    pub dash_state: DashState,
    pub dash_power: f32,
    pub dashing_timer: Timer,
    pub dash_cooldown_timer: Timer,
}

impl Default for Dasher {
    fn default() -> Self {
        Dasher {
            dash_state: DashState::ReadyToDash,
            dash_power: 30000.,
            dashing_timer: Timer::from_seconds(0.2, TimerMode::Once),
            dash_cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DashState {
    ReadyToDash,
    Dashing(Timer),
    OnCooldown(Timer),
}

impl Dasher {
    pub fn transition(&mut self, duration: Duration) {
        match &mut self.dash_state {
            DashState::ReadyToDash => self.dash_state = DashState::Dashing(Timer::from_seconds(0.2, TimerMode::Once)),
            DashState::Dashing(timer) => if timer.finished() { 
                                                    self.dash_state = DashState::OnCooldown(Timer::from_seconds(0.5, TimerMode::Once));
                                                } else { 
                                                    timer.tick(duration);
                                                },
            DashState::OnCooldown(timer) => if timer.finished() {
                                                    self.dash_state = DashState::ReadyToDash;
                                                } else {
                                                    timer.tick(duration);
                                                }
        }
    }

    pub fn is_dashing(&self) -> bool {
        match &self.dash_state {
            DashState::Dashing(_) => true,
            _ => false
        }
    }
}

impl Default for HorizontalMover {
    fn default() -> Self {
        HorizontalMover {
            is_horizontal_moving: false,

            walk_speed: 10000.,
            walk_acc: 1000.0,
            walk_dec: 1000.0,
            walk_turn: 1000.0,

            run_speed: 15000.,
            run_acc: 500.0,
            run_dec: 1000.0,
            run_turn: 800.0,

            air_speed: 10000.,
            air_acc: 1000.0,
            air_dec: 1000.0,
            air_turn: 1000.0,

            current_speed: 0.0,

            /* dash_state: DashState::ReadyToDash,
            dash_power: 30000.,
            dashing_timer: Timer::from_seconds(0.2, TimerMode::Once),
            dash_cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once), */

            facing_direction: FacingDirection::Left,
        }
    }
}

#[derive(Clone, Component)]
pub struct VerticalMover {
    pub jump_height: f32,
    pub time_to_jump_apex: f32,
    pub down_grav_mult: f32,

    pub jump_count: i32,
    pub max_jump_count: i32,

    pub is_wall_sliding: bool,
    pub wall_slide_speed: f32,
    pub in_wall_slide_coyote_time: bool,
    pub wall_slide_coyote_timer: Timer,

    pub can_wall_jump: bool,
    pub is_wall_jumping: bool,
    pub wall_jump_direction: FacingDirection,
    //pub wall_jump_coyote_timer: Timer,
    pub wall_jump_timer: Timer,
    pub wall_jump_cooldown_timer: Timer,

    //pub temp_counter: i32,
}

/* pub const WALL_SLIDE_COYOTE_TIME: f32 = 0.01;
pub const WALL_JUMP_COYOTE_TIME: f32 = 0.5;
pub const WALL_JUMP_TIME: f32 = 0.2; */
//pub const TEST_TIME: f32 = 0.5;

impl Default for VerticalMover {
    fn default() -> Self {
        const JUMPS: i32 = 1;
        VerticalMover { 
            //height 225000 clears 3 blocks
            jump_height: 225000.,
            time_to_jump_apex: 20.,
            down_grav_mult: 1.5,

            jump_count: JUMPS,
            max_jump_count: JUMPS,

            is_wall_sliding: false,
            wall_slide_speed: 10.,
            in_wall_slide_coyote_time: false,
            wall_slide_coyote_timer: Timer::from_seconds(0.2, TimerMode::Once),

            can_wall_jump: true,
            is_wall_jumping: false,
            wall_jump_direction: FacingDirection::Left,
            wall_jump_timer: Timer::from_seconds(0.5, TimerMode::Once),
            wall_jump_cooldown_timer: Timer::from_seconds(0.1, TimerMode::Once),


            //temp_counter: 0,
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
