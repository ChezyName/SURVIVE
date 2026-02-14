mod player;
mod projectile;
mod config;
mod enemy;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_plugins(player::PlayerPlugin)// Player
        .add_plugins(projectile::ProjectilePlugin)// Projectiles
        .add_plugins(enemy::EnemyPlugin)// Enemies
        .add_systems(Startup, spawn_test_enemy)// Spawn enemy to test
        .run();
}

fn spawn_test_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials,  0.0, 3);
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, 30.0, 4);
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, -30.0, 5);

    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, -90.0, 2);
}
