use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const ACCURACY_INCREASE: f32 = 2.5;
const COST: usize = 75;

#[derive(Clone, Default)]
pub struct Accuracy;

impl Item for Accuracy {
    fn name(&self) -> String {
        "Sniper Scope".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self, player: &mut Player) -> String {
        format!("Reduces Spread by {}°", format(ACCURACY_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        let min_spread = (player.pellets as f32 - 1.0).max(0.0);
        player.bullet_spread = (player.bullet_spread - ACCURACY_INCREASE).max(min_spread);
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { return true; }
}

inventory::submit!(ItemFactory(|| Box::new(Accuracy::default())));