use bevy::prelude::*;

use std::time::Duration;

#[derive(Clone, Bundle, Default)]
pub struct MovementBundle {
    pub facing: Facing,
    pub runner: Runner,
    pub dasher: Dasher,

    pub vertical_mover: VerticalMover,
}

#[derive(Clone, Component)]
pub struct Facing(pub FacingDirection);

impl Default for Facing {
    fn default() -> Self {
        Facing(FacingDirection::Right)
    }
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

#[derive (Clone, Component)]
pub struct Runner {
    pub current_speed: f32,
    pub run_speed: Speed,
    pub sprint_speed: Speed,
    pub air_speed: Speed,
}

#[derive(Clone, Copy)]
pub struct Speed {
    pub max: f32,
    pub accel: f32,
    pub decel: f32,
    pub turn: f32
}

impl Speed {
    fn new(max: f32, accel: f32, decel: f32, turn: f32) -> Self {
        Speed{max, accel, decel, turn}
    }
}

impl Default for Runner {
    fn default() -> Self {
        Runner {
            current_speed: 0.,
            run_speed: Speed::new(10000., 1000.0, 1000.0, 1000.0,),
            sprint_speed: Speed::new(15000., 500.0, 1000.0, 800.0,),
            air_speed: Speed::new( 10000., 1000.0, 1000.0, 1000.0,),
        }
    }
}

#[derive(Clone, Component)]
pub struct Dasher {
    pub dash_state: DashState,
    pub dash_power: f32,
}

impl Default for Dasher {
    fn default() -> Self {
        Dasher {
            dash_state: DashState::ReadyToDash,
            dash_power: 30000.,
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
            wall_jump_cooldown_timer: Timer::from_seconds(0.0, TimerMode::Once),


            //temp_counter: 0,
        }
    }
}