use bevy::time::{Timer, TimerMode};
use super::{Item, ItemFactory};
use crate::{config, player::Player};

const COST: usize = 350;
const TIME_PER_LEVEL: [f32; 5] = [12.0, 8.5, 5.0, 3.0, 1.5];

#[derive(Default, Clone)]
pub struct Wave;

fn current_level(player: &Player) -> usize {
    TIME_PER_LEVEL.iter()
        .position(|&t| t == player.wave_time)
        .map(|i| i + 1)
        .unwrap_or(0)
}

impl Item for Wave {
    fn name(&self) -> String { "Wave".to_string() }

    fn cost(&self) -> usize { 350 }

    fn description(&self, player: &mut Player) -> String {
        let next_level = current_level(player).clamp(0, TIME_PER_LEVEL.len() - 1);
        format!("Spawn an AOE Wave every {}s", config::format(TIME_PER_LEVEL[next_level]))
    }

    fn apply(&self, player: &mut Player) {
        let next_level = current_level(player);
        if next_level < TIME_PER_LEVEL.len() {
            let new_time = TIME_PER_LEVEL[next_level];
            player.wave_time = new_time;
            player.wave_timer = Timer::from_seconds(new_time, TimerMode::Repeating);
        }
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool {
        current_level(player) < TIME_PER_LEVEL.len()
    }
}

inventory::submit!(ItemFactory(|| Box::new(Wave::default())));