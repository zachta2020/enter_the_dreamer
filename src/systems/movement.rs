use crate::components::movement::*;
use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::movement::Direction;
use std::f32::{INFINITY, NEG_INFINITY};
use std::time::Duration;

pub fn horizontal_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut FacingDirection,
            &mut Runner,
            &WallJumper,
            &GroundDetection,
            &Dasher,
        ),
        With<Player>,
    >,
) {
    for (
        mut velocity,
        mut facing_direction,
        mut runner,
        wall_jumper,
        ground_detection,
        dasher,
    ) in &mut query
    {
        use DashState::*;
        use WallJumpState::*;

        // ORIGINAL
        // let mut direction: f32 = 0.;
        // if !dasher.is_dashing() && !wall_jumper.is_wall_jumping() {
        //     if input.pressed(KeyCode::Right) {
        //         facing_direction.0 = Direction::Right;
        //         direction = 1.;
        //     } else if input.pressed(KeyCode::Left) {
        //         facing_direction.0 = Direction::Left;
        //         direction = -1.;
        //     } else {
        //         direction = 0.;
        //     }
        // }

        // NEW
        let direction: f32 = if let (Dashing(_), WallJumping(_)) =
            (&dasher.dash_state, &wall_jumper.wall_jump_state)
        {
            if input.pressed(KeyCode::Right) {
                facing_direction.0 = Direction::Right;
                1.
            } else if input.pressed(KeyCode::Left) {
                facing_direction.0 = Direction::Left;
                -1.
            } else {
                0.0
            }
        } else {
            0.0
        };

        let horizontal_speed: Speed;
        if !ground_detection.on_ground {
            //if in the air
            if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
                runner.air_speed.max = runner.sprint_speed.max;
            } else {
                runner.air_speed.max = runner.run_speed.max;
            }
            horizontal_speed = runner.air_speed;
        } else if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight)
        {
            //if sprinting
            horizontal_speed = runner.sprint_speed;
        } else {
            //if normal running
            horizontal_speed = runner.run_speed;
        };

        // ORIGINAL
        // let speed_change: f32;
        // if direction != 0.0 {
        //     if direction.signum() != runner.current_speed.signum() {
        //         //if the player changes direction
        //         speed_change = horizontal_speed.turn;
        //     } else {
        //         //if the player keeps going in the same direction
        //         speed_change = horizontal_speed.accel;
        //     }
        // } else {
        //     //slow down when the player lets go of move left and/or move right
        //     speed_change = horizontal_speed.decel;
        // }

        // IDIOMATIC IF ELSE
        // let speed_change = if direction == 0 {
        //     // slow down when the player lets go of move left and/or move right
        //     horizontal_speed.decel
        // } else {
        //     if direction.signum() != runner.current_speed.signum() as i32 {
        //         // if the player changes direction
        //         horizontal_speed.turn
        //     } else {
        //         // if the player keeps going in the same direction
        //         horizontal_speed.accel
        //     }
        // };

        // NEW
        let is_stopping = direction == 0.;
        let is_turning_around = direction.signum() != runner.current_speed.signum();
        let speed_change = direction
            * match (is_stopping, is_turning_around) {
                (true, _) => horizontal_speed.decel,
                (_, true) => horizontal_speed.turn,
                (_, false) => horizontal_speed.accel,
            };

        // ORIGINAL
        // if direction == 0.0 {
        //     //if decelerating
        //     if runner.current_speed > 0.0 {
        //         //decelerating from a positive velocity
        //         runner.current_speed -= speed_change;
        //         if runner.current_speed <= 0.0 {
        //             runner.current_speed = 0.0
        //         }
        //     } else {
        //         //decelerating from a negative velocity
        //         runner.current_speed += speed_change;
        //         if runner.current_speed >= 0.0 {
        //             runner.current_speed = 0.0
        //         }
        //     }
        // } else {
        //     //if accelerating or turning
        //     runner.current_speed += direction * speed_change;
        //     if runner.current_speed.abs() >= horizontal_speed.max {
        //         runner.current_speed = direction * horizontal_speed.max;
        //     }
        // }

        // NEW
        let bounds = match (is_stopping, direction as i32) {
            (true, -1) => (0.0, INFINITY),
            (true, 1) => (NEG_INFINITY, 0.0),
            _ => (-horizontal_speed.max, horizontal_speed.max),
        };
        runner.current_speed =
            (runner.current_speed + speed_change).clamp(bounds.0, bounds.1);

        velocity.linvel.x = runner.current_speed * time.delta_seconds();
        //println!("Current Velocity: {}", velocity.linvel.x);
    }
}

pub fn horizontal_dash(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Dasher, &FacingDirection), With<Player>>,
) {
    for (mut velocity, mut dasher, facing) in &mut query {
        let direction = if facing.0 == Direction::Left { -1. } else { 1. };
        match dasher.dash_state {
            DashState::ReadyToDash => {
                if input.just_pressed(KeyCode::ControlLeft) {
                    dasher.transition(Duration::ZERO);
                }
            }
            DashState::Dashing(_) => {
                velocity.linvel =
                    Vec2::new(direction * dasher.dash_power * time.delta_seconds(), 0.0);
                dasher.transition(time.delta());
            }
            DashState::OnCooldown(_) => dasher.transition(time.delta()),
        }
    }
}

pub fn set_player_gravity(
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut GravityScale, &Dasher, &Jumper, &Velocity), With<Player>>,
) {
    for (mut gravity_scale, dasher, jumper, velocity) in &mut query {
        let new_gravity = (-2. * jumper.jump_height)
            / (jumper.time_to_jump_apex * jumper.time_to_jump_apex);

        let gravity_mult = if dasher.is_dashing() {
            0.0
        } else if velocity.linvel.y < -0.01 {
            //falling
            jumper.down_grav_mult
        } else {
            //jumping or standing on ground
            1.0
        };

        gravity_scale.0 = (new_gravity / rapier_config.gravity.y) * gravity_mult;
    }
}

pub fn vertical_jump(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumper, &GroundDetection), With<Player>>,
) {
    for (mut velocity, mut jumper, ground_detection) in &mut query {
        if input.just_pressed(KeyCode::Space)
            && (ground_detection.on_ground || jumper.jump_count > 0)
        {
            jumper.jump_count -= 1;
            velocity.linvel.y =
                jumper.get_jump_speed(velocity.linvel.y, 1.0) * time.delta_seconds();
        }
    }
}

pub fn set_jumps(
    mut query: Query<(&mut Jumper, &Velocity, &GroundDetection), With<Player>>,
) {
    for (mut jumper, velocity, ground_detection) in &mut query {
        if ground_detection.on_ground && velocity.linvel.y == 0.0 {
            jumper.jump_count = jumper.max_jump_count;
        } else if !ground_detection.on_ground
            && jumper.jump_count == jumper.max_jump_count
        {
            jumper.jump_count -= 1;
        }
    }
}

pub fn wall_slide_and_jump(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut WallJumper,
            &mut FacingDirection,
            &mut Jumper,
            &GroundDetection,
            &WallDetection,
        ),
        With<Player>,
    >,
) {
    for (
        mut velocity,
        mut wall_jumper,
        mut facing,
        mut jumper,
        ground_detection,
        wall_detection,
    ) in &mut query
    {
        let input_states = (
            input.just_pressed(KeyCode::Space),
            input.pressed(KeyCode::Left),
            input.pressed(KeyCode::Right),
        );
        let detection_states = (
            ground_detection.on_ground,
            wall_detection.on_wall_left,
            wall_detection.on_wall_right,
        );

        match wall_jumper.wall_jump_state {
            WallJumpState::NotOnWall => {
                wall_jumper.transition(Duration::ZERO, input_states, detection_states);
            }
            WallJumpState::WallSliding => {
                if velocity.linvel.y < -1. * wall_jumper.wall_slide_speed {
                    velocity.linvel.y = -1. * wall_jumper.wall_slide_speed;
                }
                //println!("Bool states - {:?}, Wall Slide", bool_states);
                wall_jumper.transition(Duration::ZERO, input_states, detection_states);

                if wall_jumper.is_wall_jumping() {
                    wall_jumper.wall_jump_direction = facing.0.get_opposite();
                }
            }
            WallJumpState::WallSlideCoyote(_) => {
                //println!("Bool states - {:?}, Wall Slide Coyote", bool_states);
                wall_jumper.transition(time.delta(), input_states, detection_states);

                if wall_jumper.is_wall_jumping() {
                    wall_jumper.wall_jump_direction = facing.0.get_opposite();
                }
            }
            WallJumpState::WallJumping(_) => {
                if facing.0 != wall_jumper.wall_jump_direction {
                    facing.0 = facing.0.get_opposite();
                    //println!("Flip!");
                    jumper.jump_count = 0;

                    let wall_jump_speed = jumper.get_jump_speed(velocity.linvel.y, 1.25);
                    velocity.linvel.y = wall_jump_speed * time.delta_seconds();
                }

                let direction = if wall_jumper.wall_jump_direction == Direction::Right {
                    1.
                } else {
                    -1.
                };

                velocity.linvel.x = direction * 6000. * time.delta_seconds();

                //println!("Wall Jump!");
                wall_jumper.transition(time.delta(), input_states, detection_states);
            }
        }
    }
}

/* pub fn debug (
    //wall_detection_query: Query<&WallDetection, Changed<WallDetection>>
    wall_jumper_query: Query<&WallJumper>
) {
    for wall_jumper in &wall_jumper_query {
        println!("Wall Jump State - {:?}", wall_jumper.wall_jump_state);
    }
} */
