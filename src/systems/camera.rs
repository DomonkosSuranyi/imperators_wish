use bevy::prelude::*;
use crate::components::{Player, PlayerBundle};

pub fn setup(mut commands: Commands) {
    log::info!("Creating camera");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    log::info!("Camera spawned");
}

pub fn follow_player(
    player_query: Query<&GlobalTransform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        log::info!("updating camera");
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
