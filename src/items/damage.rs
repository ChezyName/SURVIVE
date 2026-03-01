use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const DAMAGE_INCREASE: f32 = 10.0;

#[derive(Clone, Default)]
pub struct Damage;

impl Item for Damage {
    fn name(&self) -> String { "Sharpness".to_string() }

    fn cost(&self) -> usize { 50 }

    fn description(&self, player: &mut Player) -> String {
        format!("Increases Damage by {}", format(DAMAGE_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        player.damage = player.damage + DAMAGE_INCREASE;
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { return true; }
}

inventory::submit!(ItemFactory(|| Box::new(Damage::default())));