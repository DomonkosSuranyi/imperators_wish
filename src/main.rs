use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod systems;
mod components;
mod resources;

fn main() {
    log::info!("Application started");
    App::new()
        .add_plugins(DefaultPlugins)

        .init_resource::<resources::SpriteResource>()

        .add_startup_system(systems::camera::setup)
        .add_startup_system(systems::sprite::initialize_sprite_resource)
        .add_startup_system(|mut commands: Commands| {
            systems::spawn_player(&mut commands, Vec2::new(0.0, 0.0));
        })

        .add_system(systems::camera::follow_player)

        .add_system_to_stage(CoreStage::PostUpdate, systems::sprite::add_sprite_to_new_sprite_id)
        .run()
}
