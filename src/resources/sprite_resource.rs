use crate::components::SpriteId;
use bevy::prelude::*;

#[derive(Clone, Default)]
pub struct SpriteResource {
    pub player: Handle<Image>,
    pub floor: Handle<Image>,
}

impl SpriteResource {
    pub fn texture_for(&self, id: SpriteId) -> Handle<Image> {
        match id {
            SpriteId::Player => self.player.clone(),
            SpriteId::Floor  => self.floor.clone(),
        }
    }
}

