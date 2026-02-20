use super::{Item, ItemFactory};
use crate::{config, player::Player};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::config::format;

//adds one pellet per
const ACCURACY_INCREASE: [f32;2] = [8.0, 2.5]; //accuracy in deg min - max (based on pellet count)
const COST: usize = 300;
const MAX_LEVEL: usize = 10;

#[derive(Default)]
pub struct Shotgun {
    level: AtomicUsize,
}

impl Clone for Shotgun {
    fn clone(&self) -> Self {
        Self {
            level: AtomicUsize::new(self.level.load(Ordering::Relaxed)),
        }
    }
}

impl Item for Shotgun {
    fn name(&self) -> String {
        "Shotgun".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        let current_lvl = self.level.load(Ordering::Relaxed);
        format!("Adds Pellet but reduces accuracy by {}", format(config::lerp(ACCURACY_INCREASE[0], ACCURACY_INCREASE[1], (current_lvl as f32/MAX_LEVEL as f32).clamp(0.0, 1.0))))
    }

    fn apply(&self, player: &mut Player) {
        let current_lvl = self.level.fetch_add(1, Ordering::Relaxed);
        player.pellets = player.pellets + 1;
        player.bullet_spread = player.bullet_spread + config::lerp(ACCURACY_INCREASE[0], ACCURACY_INCREASE[1], (current_lvl as f32/MAX_LEVEL as f32).clamp(0.0, 1.0));
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool {
        let current_lvl = self.level.load(Ordering::Relaxed);
        return current_lvl <= MAX_LEVEL
    }
}

inventory::submit!(ItemFactory(|| Box::new(Shotgun::default())));