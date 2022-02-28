use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub jetpack: Jetpack,
    pub velocity: Velocity,
    pub bounding_circle: BoundingCircle,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub sprite_id: SpriteId,
}

impl PlayerBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            health: Health(255u8),
            bounding_circle: BoundingCircle { radius: 16 },
            transform: Transform {
                translation: position.extend(1.0),
                ..Default::default()
            },
            sprite_id: SpriteId::Player,

            player: Player,
            jetpack: Jetpack::default(),
            velocity: Velocity::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Health(u8);

#[derive(Component)]
pub struct Jetpack {
    // dimension: pixel / sec^2
    // The weight of the player is constant here
    pub accelerating_power: u8,
    pub fuel: u8,
    pub tank_size: u8,
    // consumption is constant when throttle: 1l/32px

    pub is_braking: bool,
    pub auto_brake: bool,
}

impl Default for Jetpack {
    fn default() -> Self {
        Jetpack {
            accelerating_power: 32u8,
            fuel: 255u8,
            tank_size: 255u8,
            is_braking: false,
            auto_brake: true
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity(u32);

#[derive(Component)]
pub struct BoundingCircle {
    pub radius: u8
}

impl Default for BoundingCircle {
    fn default() -> Self {
        BoundingCircle { radius: 16u8 }
    }
}

#[derive(Copy, Clone, PartialEq, Component)]
pub enum SpriteId {
    Player,
    Floor,
}
