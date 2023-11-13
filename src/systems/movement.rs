use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

pub fn horizontal_movement (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut HorizontalMover), With<Player>>
) {
    const TURN_SPEED: f32 = 500.0;
    const ACCELERATION: f32 = 500.0;
    const DECELERATION: f32 = 800.0;

    for (mut velocity, mut horizontal_mover) in &mut query {
        let right = if input.pressed(KeyCode::Right) { 
            horizontal_mover.facing_direction = PlayerDirection::Right;
            1.
        } else { 0. };
        let left = if input.pressed(KeyCode::Left) { 
            horizontal_mover.facing_direction = PlayerDirection::Left;
            1. 
        } else { 0. };
        let direction: f32 = right - left;
        
        let horizontal_speed = if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            horizontal_mover.run_speed
        } else {
            horizontal_mover.walk_speed
        };

        let speed_change: f32;
        if direction != 0.0 {
            if direction.signum() != horizontal_mover.current_speed.signum() { //if the player changes direction
                speed_change = TURN_SPEED;
                println!("Changing Direction");
            } else { //if the player keeps going in the same direction
                speed_change = ACCELERATION;
                println!("Same Direction");
            }
        } else { //slow down when the player lets go of move left or move right
            speed_change = DECELERATION;
            println!("Slowing Down");
        }

        if direction == 0.0 { //if decelerating
            if horizontal_mover.current_speed > 0.0 { //decelerating from a positive velocity
                horizontal_mover.current_speed -= speed_change;
                if horizontal_mover.current_speed <= 0.0 {
                    horizontal_mover.current_speed = 0.0
                }
            } else { //decelerating from a negative velocity
                horizontal_mover.current_speed += speed_change;
                if horizontal_mover.current_speed >= 0.0 {
                    horizontal_mover.current_speed = 0.0
                }
            }
        } else { //if accelerating or turning
            horizontal_mover.current_speed += direction * speed_change;
            if horizontal_mover.current_speed.abs() >= horizontal_speed {
                horizontal_mover.current_speed = direction * horizontal_speed;
            }
        }

        velocity.linvel.x = horizontal_mover.current_speed * time.delta_seconds();
        println!("Current Velocity: {}", velocity.linvel.x);
    }
}

pub fn horizontal_movement_no_acc (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut HorizontalMover), With<Player>>
) {
    for (mut velocity, mut horizontal_mover) in &mut query {
        let right = if input.pressed(KeyCode::Right) && !horizontal_mover.is_dashing { 
            horizontal_mover.facing_direction = PlayerDirection::Right;
            1.
        } else { 0. };
        let left = if input.pressed(KeyCode::Left) && !horizontal_mover.is_dashing { 
            horizontal_mover.facing_direction = PlayerDirection::Left;
            1. 
        } else { 0. };
        let direction: f32 = right - left;
    
        let horizontal_speed = if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            horizontal_mover.run_speed
        } else {
        horizontal_mover.walk_speed
        };

        velocity.linvel.x = direction * horizontal_speed * time.delta_seconds();
    }
}

pub fn horizontal_dash (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut HorizontalMover, &mut GravityScale), With<Player>>
) {
    for (mut velocity, mut horizontal_mover, mut gravity_scale) in &mut query {
        let direction = if horizontal_mover.facing_direction == PlayerDirection::Left { -1. } else { 1. };
        //initiate the dash
        if input.just_pressed(KeyCode::ControlLeft) && horizontal_mover.can_dash{
            horizontal_mover.can_dash = false;
            horizontal_mover.is_dashing = true;

            horizontal_mover.predash_gravity = gravity_scale.0;
            gravity_scale.0 = 0.0;

            horizontal_mover.dashing_timer.reset();
        }
        //while dashing
        if horizontal_mover.is_dashing {
            if !horizontal_mover.dashing_timer.just_finished() {
                velocity.linvel.x = direction * horizontal_mover.dash_power * time.delta_seconds();
                velocity.linvel.y = 0.0;
                horizontal_mover.dashing_timer.tick(time.delta());
            } else {
                gravity_scale.0 = horizontal_mover.predash_gravity;
                horizontal_mover.is_dashing = false;

                horizontal_mover.dash_cooldown_timer.reset();
            }
        }
        //after the dash
        if !horizontal_mover.can_dash{

            if !horizontal_mover.dash_cooldown_timer.just_finished() {
                horizontal_mover.dash_cooldown_timer.tick(time.delta());
                //println!("COOLING DOWN DASH");
            }

            horizontal_mover.can_dash = true;
        }
    }
}

pub fn vertical_jump (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut VerticalMover, &GroundDetection), With<Player>>
) {
    for (mut velocity, mut vertical_mover, ground_detection) in &mut query {
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || vertical_mover.jump_count > 0) {
            vertical_mover.jump_count -= 1;
            velocity.linvel.y = vertical_mover.jump_speed * time.delta_seconds();
        }
    }
}

pub fn wall_jump (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &VerticalMover, &HorizontalMover, &GroundDetection, &WallDetection), With<Player>>
) {
    for (mut velocity, vertical_mover, horizontal_mover, ground_detection, wall_detection) in &mut query {
        if input.just_pressed(KeyCode::Space) && (wall_detection.on_wall && !ground_detection.on_ground) {
            velocity.linvel.x = vertical_mover.jump_speed * time.delta_seconds();
            velocity.linvel.y = vertical_mover.jump_speed * time.delta_seconds();
        }
    }
}

pub fn refresh_jumps(
    mut query: Query<(&mut VerticalMover, &mut HorizontalMover, &Velocity, &GroundDetection), With<Player>>,
) {
    for (mut vertical_mover, mut horizontal_mover, velocity, ground_detection) in &mut query {
        if ground_detection.on_ground && velocity.linvel.y == 0.0 {
            vertical_mover.jump_count = vertical_mover.max_jump_count;
        }
    }
}