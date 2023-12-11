use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::components::movement::*;

use std::time::Duration;

pub fn horizontal_movement (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Facing, &mut Runner, &VerticalMover, &GroundDetection, &Dasher), With<Player>>
) {
    for (mut velocity, mut facing, mut runner, vertical_mover, ground_detection, dasher) in &mut query {
        let right = if input.pressed(KeyCode::Right) && !dasher.is_dashing() && !vertical_mover.is_wall_jumping { 
            facing.0 = FacingDirection::Right;
            1.
        } else { 0. };
        let left = if input.pressed(KeyCode::Left) && !dasher.is_dashing() && !vertical_mover.is_wall_jumping { 
            facing.0 = FacingDirection::Left;
            1. 
        } else { 0. };
        let direction: f32 = right - left;

        let horizontal_speed: Speed;
        if !ground_detection.on_ground { //if in the air
            horizontal_speed = runner.air_speed;
        } else if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) { //if sprinting
            horizontal_speed = runner.sprint_speed;
        } else { //if normal running
            horizontal_speed = runner.run_speed;
        };

        let speed_change: f32;
        if direction != 0.0 {
            if direction.signum() != runner.current_speed.signum() { //if the player changes direction
                speed_change = horizontal_speed.turn;
            } else { //if the player keeps going in the same direction
                speed_change = horizontal_speed.accel;
            }
        } else { //slow down when the player lets go of move left and/or move right
            speed_change = horizontal_speed.decel;
        }

        if direction == 0.0 { //if decelerating
            if runner.current_speed > 0.0 { //decelerating from a positive velocity
                runner.current_speed -= speed_change;
                if runner.current_speed <= 0.0 {
                    runner.current_speed = 0.0
                }
            } else { //decelerating from a negative velocity
                runner.current_speed += speed_change;
                if runner.current_speed >= 0.0 {
                    runner.current_speed = 0.0
                }
            }
        } else { //if accelerating or turning
            runner.current_speed += direction * speed_change;
            if runner.current_speed.abs() >= horizontal_speed.max {
                runner.current_speed = direction * horizontal_speed.max;
            }
        }

        velocity.linvel.x = runner.current_speed * time.delta_seconds();
        //println!("Current Velocity: {}", velocity.linvel.x);
    }
}

pub fn horizontal_dash (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Dasher, &Facing), With<Player>>
) {
    for (mut velocity, mut dasher, facing) in &mut query {
        let direction = if facing.0 == FacingDirection::Left { -1. } else { 1. };
        match dasher.dash_state {
            DashState::ReadyToDash => if input.just_pressed(KeyCode::ControlLeft) {
                dasher.transition(Duration::ZERO);
            }, 
            DashState::Dashing(_) => { 
                velocity.linvel = Vec2::new(direction * dasher.dash_power * time.delta_seconds(), 0.0);
                dasher.transition(time.delta());
            },
            DashState::OnCooldown(_) => dasher.transition(time.delta())
        }
    }
}

pub fn set_player_gravity(
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut GravityScale, &Dasher, &VerticalMover, &Velocity), With<Player>>
) {
    for(mut gravity_scale, dasher, vertical_mover, velocity) in &mut query {
        let new_gravity = (-2. * vertical_mover.jump_height) / (vertical_mover.time_to_jump_apex * vertical_mover.time_to_jump_apex);

        let gravity_mult = if dasher.is_dashing() {
            0.0
        } else if velocity.linvel.y < -0.01 { //falling
            vertical_mover.down_grav_mult
        } else { //jumping or standing on ground
            1.0
        };

        gravity_scale.0 = (new_gravity / rapier_config.gravity.y) * gravity_mult;
    }
}

pub fn vertical_jump (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut Velocity, &mut VerticalMover, &GroundDetection, &GravityScale), With<Player>>
) {
    for (mut velocity, mut vertical_mover, ground_detection, gravity_scale) in &mut query {
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || vertical_mover.jump_count > 0) {
            vertical_mover.jump_count -= 1;

            let jump_power = (-2. * rapier_config.gravity.y * gravity_scale.0 * vertical_mover.jump_height).sqrt();
            /* println!("Jump Power: {}", jump_power);
            println!("Gravity Scale: {}", gravity_scale.0); */

            velocity.linvel.y = jump_power * time.delta_seconds();

            //DEBUG
            /* vertical_mover.temp_counter += 1; */
            //println!("DEBUG: Jump!");
        }
    }
}

pub fn wall_jump (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut Velocity, &mut VerticalMover, &mut Facing, &GravityScale, &GroundDetection, &WallDetection), With<Player>>
) {
    for (mut velocity, mut vertical_mover, mut facing, gravity_scale, ground_detection, wall_detection) in &mut query {
         //start the wall jump
        if input.just_pressed(KeyCode::Space) && vertical_mover.is_wall_sliding && vertical_mover.can_wall_jump {
            vertical_mover.is_wall_sliding = false;
            vertical_mover.in_wall_slide_coyote_time = false;
            vertical_mover.can_wall_jump = false;
            vertical_mover.is_wall_jumping = true;

            vertical_mover.wall_jump_direction = facing.0.get_opposite();

            vertical_mover.wall_jump_timer.reset();

            let jump_power = (-2. * rapier_config.gravity.y * gravity_scale.0 * vertical_mover.jump_height).sqrt();
            velocity.linvel.y = jump_power * time.delta_seconds();
            vertical_mover.jump_count = 0;
            //println!("WALL JUMP");
        }
        //while wall jumping
        if vertical_mover.is_wall_jumping { 
            if !vertical_mover.wall_jump_timer.finished(){
                if facing.0 != vertical_mover.wall_jump_direction {
                    facing.0 = facing.0.get_opposite();
                }

                let direction = if vertical_mover.wall_jump_direction == FacingDirection::Right { 1. } else { -1. };

                velocity.linvel.x = direction * 6000. * time.delta_seconds();
                vertical_mover.wall_jump_timer.tick(time.delta());
            } else {
                vertical_mover.is_wall_jumping = false;
                vertical_mover.wall_jump_cooldown_timer.reset();

                //println!("WALL JUMP OVER");
            }
        }
        //cooldown from wall jumping
        if !vertical_mover.can_wall_jump && !vertical_mover.is_wall_jumping {
            if !vertical_mover.wall_jump_cooldown_timer.finished() {
                vertical_mover.wall_jump_cooldown_timer.tick(time.delta());
            } else {
                vertical_mover.can_wall_jump = true;
                //println!("WALL JUMP COOLDOWN OVER");
            }
        }

        //cancel wall jump early if the player hits the ground or floor
        if vertical_mover.is_wall_jumping && (ground_detection.on_ground || (wall_detection.on_wall && (
            (input.pressed(KeyCode::Left) && facing.0 == FacingDirection::Left) || 
            (input.pressed(KeyCode::Right) && facing.0 == FacingDirection::Right)
        ))) {
            vertical_mover.is_wall_jumping = false;
            vertical_mover.wall_jump_cooldown_timer.reset();

            //println!("WALL JUMP OVER");
        }
    }
}

pub fn set_jumps(
    mut query: Query<(&mut VerticalMover, &Velocity, &GroundDetection), With<Player>>,
) {
    for (mut vertical_mover, velocity, ground_detection) in &mut query {
        if ground_detection.on_ground && velocity.linvel.y == 0.0 {
            vertical_mover.jump_count = vertical_mover.max_jump_count;
        } else if !ground_detection.on_ground && vertical_mover.jump_count == vertical_mover.max_jump_count {
            vertical_mover.jump_count -= 1;
        }
    }
}

pub fn wall_slide (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut VerticalMover, &WallDetection, &GroundDetection, &Facing), With<Player>>
) {
    for (mut velocity, mut vertical_mover, wall_detection, ground_detection, facing) in &mut query {
        if wall_detection.on_wall && !ground_detection.on_ground && (
            (input.pressed(KeyCode::Left) && facing.0 == FacingDirection::Left) || 
            (input.pressed(KeyCode::Right) && facing.0 == FacingDirection::Right)
        ) {
            vertical_mover.is_wall_sliding = true;
            vertical_mover.in_wall_slide_coyote_time = false;
        } else if !ground_detection.on_ground && vertical_mover.is_wall_sliding && !vertical_mover.in_wall_slide_coyote_time {
            vertical_mover.in_wall_slide_coyote_time = true;
            vertical_mover.wall_slide_coyote_timer.reset();
        } else {
            vertical_mover.is_wall_sliding = false;
            vertical_mover.in_wall_slide_coyote_time = false;
        }

        if vertical_mover.in_wall_slide_coyote_time {
            if !vertical_mover.wall_slide_coyote_timer.finished() {
                vertical_mover.wall_slide_coyote_timer.tick(time.delta());
            } else {
                vertical_mover.in_wall_slide_coyote_time = false;
                vertical_mover.is_wall_sliding = false;
            }
        }

        if vertical_mover.is_wall_sliding {
            //println!("Wall Sliding");
            if velocity.linvel.y < -1. * vertical_mover.wall_slide_speed {
                velocity.linvel.y = -1. * vertical_mover.wall_slide_speed;  
            }
        }
    }
}