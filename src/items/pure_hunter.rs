use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config;

#[derive(Clone, Default)]
pub struct PureHunter;

impl Item for PureHunter {
    fn name(&self) -> String {
        "Pure Hunter".to_string()
    }

    fn cost(&self) -> usize { 375 }

    fn description(&self, player: &mut Player) -> String {
        format!("Become the true form of a Hunter. Upgrades bullet damage, and number of pellets.")
    }

    fn apply(&self, player: &mut Player) {
        player.bullet_pierce = player.bullet_pierce.max(1);
        player.bullet_speed = player.bullet_speed.max(350.0);
        player.bullet_spread = 45.0;
        player.pellets = 8;
        player.damage = player.damage.max(125.0);
        
        let fire_rate = config::PLAYER_MIN_FIRE_RATE * 3.0;
        player.fire_rate = fire_rate;
        player.fire_timer.set_duration(std::time::Duration::from_secs_f32(60.0/fire_rate));
        player.fire_timer.reset();
        player.fire_timer.tick(player.fire_timer.duration());
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(PureHunter::default())));