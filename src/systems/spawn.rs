use bevy::prelude::*;
use components::PlayerBundle;

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec2
) -> Entity {
    commands.spawn_bundle(PlayerBundle::new(position)).id()
}
