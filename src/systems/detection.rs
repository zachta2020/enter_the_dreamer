use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<With<Collider>, Without<Sensor>>,
) {
    for collision_event in collisions.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

pub fn wall_detection(
    mut wall_sensors: Query<&mut WallSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<With<Collider>, Without<Sensor>>,
) {
    for collision_event in collisions.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = wall_sensors.get_mut(*e2) {
                        sensor.intersecting_wall_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = wall_sensors.get_mut(*e1) {
                        sensor.intersecting_wall_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = wall_sensors.get_mut(*e2) {
                        sensor.intersecting_wall_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = wall_sensors.get_mut(*e1) {
                        sensor.intersecting_wall_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_wall (
    mut wall_detectors: Query<&mut WallDetection>,
    wall_sensors: Query<&WallSensor, Changed<WallSensor>>,
) {
    for sensor in &wall_sensors {
        if let Ok(mut wall_detection) = wall_detectors.get_mut(sensor.wall_detection_entity) {
            match sensor.direction {
                movement::Direction::Left => { 
                    wall_detection.on_wall_left = !sensor.intersecting_wall_entities.is_empty();
                }, 
                movement::Direction::Right => {
                    wall_detection.on_wall_right = !sensor.intersecting_wall_entities.is_empty();
                }
            }
        }
    }
}