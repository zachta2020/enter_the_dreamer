use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

pub fn horizontal_movement (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut HorizontalMover, &GroundDetection), With<Player>>
) {
    for (mut velocity, mut horizontal_mover, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::Right) && !horizontal_mover.is_dashing { 
            horizontal_mover.facing_direction = PlayerDirection::Right;
            1.
        } else { 0. };
        let left = if input.pressed(KeyCode::Left) && !horizontal_mover.is_dashing { 
            horizontal_mover.facing_direction = PlayerDirection::Left;
            1. 
        } else { 0. };
        let direction: f32 = right - left;
        
        let horizontal_speed: f32;
        let horizontal_acc: f32;
        let horizontal_dec: f32;
        let horizontal_turn: f32;

        if !ground_detection.on_ground { //if in the air
            horizontal_speed = horizontal_mover.air_speed;
            horizontal_acc = horizontal_mover.air_acc;
            horizontal_dec = horizontal_mover.air_dec;
            horizontal_turn = horizontal_mover.air_turn;
        } else if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) { //if running
            horizontal_speed = horizontal_mover.run_speed;
            horizontal_acc = horizontal_mover.run_acc;
            horizontal_dec = horizontal_mover.run_dec;
            horizontal_turn = horizontal_mover.run_turn;
        } else { //if walking
            horizontal_speed = horizontal_mover.walk_speed;
            horizontal_acc = horizontal_mover.walk_acc;
            horizontal_dec = horizontal_mover.walk_dec;
            horizontal_turn = horizontal_mover.walk_turn;
        };

        let speed_change: f32;
        if direction != 0.0 {
            if direction.signum() != horizontal_mover.current_speed.signum() { //if the player changes direction
                speed_change = horizontal_turn;
                //println!("Changing Direction");
            } else { //if the player keeps going in the same direction
                speed_change = horizontal_acc;
                //println!("Same Direction");
            }
        } else { //slow down when the player lets go of move left and/or move right
            speed_change = horizontal_dec;
            //println!("Slowing Down");
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
        //println!("Current Velocity: {}", velocity.linvel.x);
    }
}

/* pub fn horizontal_movement_no_acc (
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
} */

pub fn horizontal_dash (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut HorizontalMover), With<Player>>
) {
    for (mut velocity, mut horizontal_mover) in &mut query {
        let direction = if horizontal_mover.facing_direction == PlayerDirection::Left { -1. } else { 1. };
        //initiate the dash
        if input.just_pressed(KeyCode::ControlLeft) && horizontal_mover.can_dash{
            horizontal_mover.can_dash = false;
            horizontal_mover.is_dashing = true;

            horizontal_mover.dashing_timer.reset();
        }
        //while dashing
        if horizontal_mover.is_dashing {
            if !horizontal_mover.dashing_timer.just_finished() {
                velocity.linvel.x = direction * horizontal_mover.dash_power * time.delta_seconds();
                velocity.linvel.y = 0.0;
                horizontal_mover.dashing_timer.tick(time.delta());
                //println!("DASHING");
            } else {
                horizontal_mover.is_dashing = false;
                horizontal_mover.dash_cooldown_timer.reset();
            }
        }
        //after the dash
        if !horizontal_mover.can_dash{
            if !horizontal_mover.dash_cooldown_timer.just_finished() {
                horizontal_mover.dash_cooldown_timer.tick(time.delta());
                //println!("COOLDOWN");
            } else {
                horizontal_mover.can_dash = true;
            }
        }
    }
}

pub fn set_player_gravity(
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut GravityScale, &HorizontalMover, &VerticalMover, &Velocity), With<Player>>
) {
    for(mut gravity_scale, horizontal_mover, vertical_mover, velocity) in &mut query {
        let new_gravity = (-2. * vertical_mover.jump_height) / (vertical_mover.time_to_jump_apex * vertical_mover.time_to_jump_apex);

        let gravity_mult = if horizontal_mover.is_dashing {
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
    mut query: Query<(&mut Velocity, &mut VerticalMover, &mut HorizontalMover, &GroundDetection, &GravityScale), With<Player>>
) {
    for (mut velocity, mut vertical_mover, mut horizontal_mover, ground_detection, gravity_scale) in &mut query {
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || vertical_mover.jump_count > 0) {
            vertical_mover.jump_count -= 1;
            if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
                horizontal_mover.air_speed = horizontal_mover.run_speed;
            } else {
                horizontal_mover.air_speed = horizontal_mover.walk_speed;
            }

            let jump_power = (-2. * rapier_config.gravity.y * gravity_scale.0 * vertical_mover.jump_height).sqrt();
            /* println!("Jump Power: {}", jump_power);
            println!("Gravity Scale: {}", gravity_scale.0); */

            velocity.linvel.y = jump_power * time.delta_seconds();
            println!("JUMP!")
        }
    }
}

/* pub fn wall_jump (
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &VerticalMover, &HorizontalMover, &GroundDetection, &WallDetection), With<Player>>
) {
    for (mut velocity, vertical_mover, horizontal_mover, ground_detection, wall_detection) in &mut query {
        if input.just_pressed(KeyCode::Space) && (wall_detection.on_wall && !ground_detection.on_ground) {
            velocity.linvel.x = vertical_mover.jump_power * time.delta_seconds();
            velocity.linvel.y = vertical_mover.jump_power * time.delta_seconds();
        }
    }
} */

pub fn refresh_jumps(
    mut query: Query<(&mut VerticalMover, &Velocity, &GroundDetection), With<Player>>,
) {
    for (mut vertical_mover, velocity, ground_detection) in &mut query {
        if ground_detection.on_ground && velocity.linvel.y == 0.0 {
            vertical_mover.jump_count = vertical_mover.max_jump_count;
        }
    }
}