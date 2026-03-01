use super::{Item, ItemFactory};
use crate::{config, player::Player};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::config::format;

//adds one pellet per
const ACCURACY_INCREASE: [f32;2] = [8.0, 2.5]; //accuracy in deg min - max (based on pellet count)
const FIRE_RATE_DECREASE: f32 = 50.0;
const MAX_LEVEL: usize = 8;

#[derive(Default, Clone)]
pub struct Shotgun;

impl Item for Shotgun {
    fn name(&self) -> String { "Shotgun".to_string() }

    fn cost(&self) -> usize { 300 }

    fn description(&self, player: &mut Player) -> String {
        format!("Adds Pellet but reduces Accuracy by {}° and Fire Rate by {} RPM", format(config::lerp(ACCURACY_INCREASE[0], ACCURACY_INCREASE[1], (player.pellets.max(0) as f32/MAX_LEVEL as f32).clamp(0.0, 1.0))), format(FIRE_RATE_DECREASE))
    }

    fn apply(&self, player: &mut Player) {
        let current_lvl = player.pellets;
        let fire_rate = (player.fire_rate - FIRE_RATE_DECREASE).max(config::PLAYER_MIN_FIRE_RATE);
        player.fire_rate = fire_rate;
        player.fire_timer.set_duration(std::time::Duration::from_secs_f32(60.0/fire_rate));
        player.fire_timer.reset();
        player.fire_timer.tick(player.fire_timer.duration());
        
        player.pellets = player.pellets + 1;
        player.bullet_spread = player.bullet_spread + config::lerp(ACCURACY_INCREASE[0], ACCURACY_INCREASE[1], (current_lvl as f32/MAX_LEVEL as f32).clamp(0.0, 1.0));
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { player.pellets < MAX_LEVEL }
}

inventory::submit!(ItemFactory(|| Box::new(Shotgun::default())));