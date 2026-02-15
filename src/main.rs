mod player;
mod projectile;
mod config;
mod enemy;
mod gamestate;
mod ui;
use bevy::prelude::*;

use crate::gamestate::GameState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_plugins(player::PlayerPlugin)// Player
        .add_plugins(ui::player::PlayerUIPlugin) //Player UI
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
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, 5.0, 4);
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, -50.0, 3);
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, 15.0, 6);
    enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, -95.0, 25);

    /*
    for i in 1..=(36/2) {
        enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, i as f32 * 20.0, i / 2);
    }
    */
}