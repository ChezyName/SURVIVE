use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameState {
    pub round: usize,
    pub money: usize,
}

