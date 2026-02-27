use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const HEALTH_INCREASE: f32 = 25.0;

#[derive(Clone, Default)]
pub struct MaxHealth;

impl Item for MaxHealth {
    fn name(&self) -> String {
        "Iron Heart".to_string()
    }

    fn cost(&self) -> usize { 100 }

    fn description(&self, player: &mut Player) -> String {
        format!("Increases Max Health by {}", format(HEALTH_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        player.max_health = player.max_health + HEALTH_INCREASE;
        player.health = (player.health + HEALTH_INCREASE).min(player.max_health);
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { return true; }
}

inventory::submit!(ItemFactory(|| Box::new(MaxHealth::default())));