use bevy::prelude::*;

use crate::components::*;

pub fn player_camera (
    mut camera_query: Query<
        (
            &mut OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>
    >,
    player_query: Query<&Transform, With<Player>>
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single() {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::WindowSize(1.5);

        camera_transform.translation.x = player_translation.x;
        camera_transform.translation.y = player_translation.y + 50.;
    }
}