pub mod camera;
pub mod sprite;

use bevy::prelude::*;
use crate::components::PlayerBundle;

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec2
) -> Entity {
    commands.spawn_bundle(PlayerBundle::new(position)).id()
}
