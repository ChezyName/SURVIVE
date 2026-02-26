use bevy::prelude::*;
use std::collections::HashMap;
use crate::enemy::EnemyType;

#[derive(Resource, Default)]
pub struct GameState {
    pub round: usize,
    pub money: usize,
    pub item_counts: HashMap<String, usize>,
    pub rerolls: usize,
    pub total_enemies_killed: usize,
    pub enemies_killed: HashMap<EnemyType, usize>,
    pub playtime_secs: f32,
}

pub fn update_playtime(
    mut game_state: ResMut<GameState>,
    time: Res<Time<Real>>,
) {
    game_state.playtime_secs += time.delta_secs();
    game_state.money += (1500.0 * time.delta_secs()) as usize;
}

//turns playtime into d h m s
pub fn fmt_playtime(secs: f32) -> String {
    let total = secs as u64;
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    let d = total / 86400;

    match (d, h, m) {
        (0, 0, 0) => format!("{}s", s),
        (0, 0, _) => format!("{}m {}s", m, s),
        (0, _, _) => format!("{}h {}m {}s", h, m, s),
        (_, _, _) => format!("{}d {}h {}m {}s", d, h, m, s),
    }
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