mod player;
mod projectile;
mod config;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_plugins(player::PlayerPlugin)// Player
        .add_plugins(projectile::ProjectilePlugin)// Projectiles
        .run();
}
