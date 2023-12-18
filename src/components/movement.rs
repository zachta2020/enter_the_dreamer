use bevy::prelude::*;

use std::{time::Duration, thread::current};

#[derive(Clone, Bundle, Default)]
pub struct MovementBundle {
    pub facing: FacingDirection,
    pub runner: Runner,
    pub dasher: Dasher,

    pub jumper: Jumper,
    pub wall_jumper: WallJumper,
}

#[derive(Clone, Component)]
pub struct FacingDirection(pub Direction);

impl Default for FacingDirection {
    fn default() -> Self {
        FacingDirection(Direction::Right)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        if *self == Direction::Left {
            Direction::Right
        } else {
            Direction::Left
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
            sprint_speed: Speed::new(15000., 1000.0, 1200.0, 800.0,),
            air_speed: Speed::new( 10000., 1000.0, 1000.0, 1000.0,),
        }
    }
}

#[derive(Clone, Component)]
pub struct Dasher {
    pub dash_state: DashState,
    pub dash_power: f32,
    dash_duration: f32,
    dash_cooldown_duration: f32
}

impl Default for Dasher {
    fn default() -> Self {
        Dasher {
            dash_state: DashState::ReadyToDash,
            dash_power: 30000.,
            dash_duration: 0.2,
            dash_cooldown_duration: 0.5
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
            DashState::ReadyToDash => self.dash_state = DashState::Dashing(Timer::from_seconds(self.dash_duration, TimerMode::Once)),
            DashState::Dashing(timer) => if timer.finished() { 
                self.dash_state = DashState::OnCooldown(Timer::from_seconds(self.dash_cooldown_duration, TimerMode::Once));
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

#[derive(Component, Clone)]
pub struct Jumper {
    pub jump_height: f32,
    pub time_to_jump_apex: f32,
    pub down_grav_mult: f32,

    pub jump_count: i32,
    pub max_jump_count: i32,
}

impl Default for Jumper {
    fn default() -> Self {
        const JUMPS: i32 = 1;
        Jumper {
            //height 225000 clears 3 blocks
            jump_height: 225000.,
            time_to_jump_apex: 20.,
            down_grav_mult: 1.5,

            jump_count: JUMPS,
            max_jump_count: JUMPS,
        }
    }
}

impl Jumper {
    pub fn get_jump_speed(&self, current_vel: f32, base_mult: f32) -> f32 {
        let base_jump_speed = 2. * self.jump_height / self.time_to_jump_apex * base_mult;

        if current_vel > 0. { //jump while rising
            let val = base_jump_speed - current_vel;
            let result = if val > 0.0 { val } else { 0.0 };
            //println!("Jump Speed - {}", result);
            result
        } else if current_vel < 0. { //jump while falling
            base_jump_speed + current_vel.abs()
        } else { //jump from ground
            base_jump_speed
        }

    }
}

#[derive(Component, Clone)]
pub struct WallJumper {
    pub wall_jump_state: WallJumpState,
    pub wall_slide_speed: f32,
    pub wall_jump_direction: Direction,
    wall_slide_coyote_duration: f32,
    wall_jump_duration: f32,
}

impl Default for WallJumper {
    fn default() -> Self {
        WallJumper { 
            wall_jump_state: WallJumpState::NotOnWall, 
            wall_slide_speed: 10., 
            wall_jump_direction: Direction::Left,
            wall_slide_coyote_duration: 0.2,
            wall_jump_duration: 0.5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WallJumpState {
    NotOnWall,
    WallSliding,
    WallSlideCoyote(Timer),
    WallJumping(Timer),
}

impl WallJumper {
    pub fn transition(&mut self, duration: Duration, input_states: (bool, bool, bool), detection_states: (bool, bool, bool)) {
        let (pressed_jump, pressed_left, pressed_right) = input_states;
        let (on_ground, on_wall_left, on_wall_right) = detection_states;

        match &mut self.wall_jump_state {
            WallJumpState::NotOnWall => if !on_ground && ((on_wall_left && pressed_left) || (on_wall_right && pressed_right)) {
                self.wall_jump_state = WallJumpState::WallSliding;
            },

            WallJumpState::WallSliding => if pressed_jump { //if the player presses jump during the WallSlide
                self.wall_jump_state = WallJumpState::WallJumping(Timer::from_seconds(self.wall_jump_duration, TimerMode::Once));
            } else if (on_wall_left && !pressed_left) || (on_wall_right && !pressed_right) { //if the player stops hugging the wall but is still on the wall
                self.wall_jump_state = WallJumpState::WallSlideCoyote(Timer::from_seconds(self.wall_slide_coyote_duration, TimerMode::Once));
            } else if on_ground {
                self.wall_jump_state = WallJumpState::NotOnWall;
            },

            WallJumpState::WallSlideCoyote(timer) => if timer.just_finished() || on_ground { //Wall Jump Coyote Timer runs out
                self.wall_jump_state = WallJumpState::NotOnWall;
            } else if pressed_jump { //Jump while in Coyote time
                self.wall_jump_state = WallJumpState::WallJumping(Timer::from_seconds(self.wall_jump_duration, TimerMode::Once));
            } else if (on_wall_left && pressed_left) || (on_wall_right && pressed_right) { //Go back to wall in Coyote time
                self.wall_jump_state = WallJumpState::WallSliding;
            } else {
                timer.tick(duration);
            },

            WallJumpState::WallJumping(timer) => if timer.just_finished() || on_ground /* || 
                (self.wall_jump_direction == Direction::Left && on_wall_left) || 
                (self.wall_jump_direction == Direction::Right && on_wall_right)  */
            {
                self.wall_jump_state = WallJumpState::NotOnWall;
            } else {
                timer.tick(duration);
            },
        }
    }

    pub fn is_wall_jumping(&self) -> bool {
        match self.wall_jump_state {
            WallJumpState::WallJumping(_) => true,
            _ => false,
        }
    }
}