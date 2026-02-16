use bevy::prelude::*;
use crate::items::ItemLogic;

#[derive(Resource, Default)]
pub struct GameState {
    pub round: usize,
    pub money: usize,
    pub items: Vec<Box<dyn ItemLogic>>,
}

