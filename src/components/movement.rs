use bevy::prelude::*;

use std::time::Duration;

#[derive(Clone, Bundle, Default)]
pub struct MovementBundle {
    pub facing: Facing,
    pub runner: Runner,
    pub dasher: Dasher,

    pub jumper: Jumper,
    pub wall_jumper: WallJumper,
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
            sprint_speed: Speed::new(15000., 1000.0, 1200.0, 800.0,),
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
    pub fn get_jump_speed(&self) -> f32 {
        ((4. * self.jump_height * self.jump_height) / (self.time_to_jump_apex * self.time_to_jump_apex)).sqrt()
    }
}

#[derive(Component, Clone)]
pub struct WallJumper {
    pub wall_jump_state: WallJumpState,
    pub wall_slide_speed: f32,
    pub wall_jump_direction: FacingDirection,
}

impl Default for WallJumper {
    fn default() -> Self {
        WallJumper { 
            wall_jump_state: WallJumpState::NotOnWall, 
            wall_slide_speed: 10., 
            wall_jump_direction: FacingDirection::Left, 
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WallJumpState {
    NotOnWall,
    WallSliding,
    WallSlideCoyote(Timer),
    WallJumping(Timer),
}

impl WallJumper {
    pub fn transition(&mut self, duration: Duration, bool_states: (bool, bool, bool, bool)) {
        let (pressed_jump, on_ground, on_wall, hugging_wall) = bool_states;
        //println!("===Bool States===\nPressed Jump - {}\nOn ground - {}\nOn wall - {}\nHugging wall - {}", 
            //pressed_jump, on_ground, on_wall, hugging_wall);
        match &mut self.wall_jump_state {
            WallJumpState::NotOnWall => { 
                if on_wall && !on_ground && hugging_wall {
                    self.wall_jump_state = WallJumpState::WallSliding;
                    //println!("NotOnWall => WallSliding");
                }
            },

            WallJumpState::WallSliding => if pressed_jump { //if the player presses jump during the WallSlide
                self.wall_jump_state = WallJumpState::WallJumping(Timer::from_seconds(0.5, TimerMode::Once));
                //println!("WallSliding => WallJumping");
            } else if !on_ground { //if the player stops wall sliding by touching the floor
                self.wall_jump_state = WallJumpState::WallSlideCoyote(Timer::from_seconds(0.2, TimerMode::Once));
                //println!("WallSliding => WallSlideCoyote");
            } else { //if the player stops wall sliding in the air
                self.wall_jump_state = WallJumpState::NotOnWall;
                //println!("WallSliding => NotOnWall");
            },

            WallJumpState::WallSlideCoyote(timer) => if timer.just_finished() || on_ground { //Wall Jump Coyote Timer runs out
                self.wall_jump_state = WallJumpState::NotOnWall;
                //println!("WallSlideCoyote => NotOnWall");
            } else if pressed_jump { //Jump while in Coyote time
                    self.wall_jump_state = WallJumpState::WallJumping(Timer::from_seconds(0.5, TimerMode::Once));
                    //println!("WallSlideCoyote => WallJumping");
            } else if on_wall && hugging_wall { //Go back to wall in Coyote time
                    self.wall_jump_state = WallJumpState::WallSliding;
                    //println!("WallSlideCoyote => WallSliding");
            } else {
                    timer.tick(duration);
            },

            WallJumpState::WallJumping(timer) => if timer.just_finished() || on_ground {
                self.wall_jump_state = WallJumpState::NotOnWall;
                //println!("WallJumping => NotOnWall")
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