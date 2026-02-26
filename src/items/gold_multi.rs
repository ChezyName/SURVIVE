use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

#[derive(Clone, Default)]
pub struct PassiveIncome;

impl Item for PassiveIncome {
    fn name(&self) -> String {
        "Double-Up".to_string()
    }

    fn cost(&self) -> usize { 125 }

    fn description(&self, player: &mut Player) -> String {
        format!("Generate Double Gold on Killing Enemies")
    }

    fn apply(&self, player: &mut Player) {
        player.gold_multi = 2.0;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(PassiveIncome::default())));