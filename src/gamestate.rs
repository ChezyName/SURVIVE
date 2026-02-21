use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GameState {
    pub round: usize,
    pub money: usize,
    pub item_counts: HashMap<String, usize>,
    pub rerolls: usize,
}

impl GameState {
    pub fn item_count(&self, name: &str) -> usize {
        self.item_counts.get(name).copied().unwrap_or(0)
    }

    pub fn has_item(&self, name: &str) -> bool {
        self.item_count(name) > 0
    }

    pub fn record_purchase(&mut self, name: String) {
        *self.item_counts.entry(name).or_insert(0) += 1;
    }
}