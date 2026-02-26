use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config;

#[derive(Clone, Default)]
pub struct PureSniper;

impl Item for PureSniper {
    fn name(&self) -> String {
        "Pure Sniper".to_string()
    }

    fn cost(&self) -> usize { 375 }

    fn description(&self, player: &mut Player) -> String {
        format!("Become the true form of a Sniper. Upgrades bullet speed, pierce, and damage.")
    }

    fn apply(&self, player: &mut Player) {
        player.bullet_pierce = player.bullet_pierce.max(3);
        player.bullet_speed = player.bullet_speed.max(1750.0);
        player.crit_percent = 100.0;
        player.bullet_spread = 0.0;
        player.damage = player.damage.max(125.0);
        
        let fire_rate = config::PLAYER_MIN_FIRE_RATE;
        player.fire_rate = fire_rate;
        player.fire_timer.set_duration(std::time::Duration::from_secs_f32(60.0/fire_rate));
        player.fire_timer.reset();
        player.fire_timer.tick(player.fire_timer.duration());
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(PureSniper::default())));