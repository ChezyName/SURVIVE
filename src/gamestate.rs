use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameState {
    pub round: u32,
    pub money: u32,
}

