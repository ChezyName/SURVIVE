use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config;

const DAMAGE_INCREASE: f32 = 10.0;
const COST: usize = 50;

#[derive(Clone, Default)]
pub struct Damage;

impl Item for Damage {
    fn name(&self) -> String {
        "Damage".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Increases Damage by {}", DAMAGE_INCREASE)
    }

    fn apply(&self, player: &mut Player) {
        player.damage = player.damage + DAMAGE_INCREASE;
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { return true; }
}

inventory::submit!(ItemFactory(|| Box::new(Damage::default())));