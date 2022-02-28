use crate::components::SpriteId;
use crate::resources::SpriteResource;
use bevy::prelude::*;

pub fn add_sprite_to_new_sprite_id(
    mut commands: Commands,
    entities_to_add_sprite: Query<
        (Entity, &SpriteId),
        (Added<SpriteId>, Without<TextureAtlasSprite>),
    >,
    sprite_resource: Res<SpriteResource>,
) {
    entities_to_add_sprite
        .iter()
        .map(|(entity, &sprite_id)| {
            let texture = sprite_resource.texture_for(sprite_id);
            (entity, texture)
        })
        .for_each(|(entity, texture)| {
            commands
                .entity(entity)
                .insert(texture)
                .insert(Sprite::default())
                .insert(Visibility::default());
        });
}

pub fn initialize_sprite_resource(
    mut sprite_resource: ResMut<SpriteResource>,
    asset_server: Res<AssetServer>,
) {
    sprite_resource.player = asset_server.load("player.png");
    sprite_resource.floor = asset_server.load("floor.png");
}
