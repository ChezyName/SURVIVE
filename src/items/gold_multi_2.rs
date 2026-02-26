use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

#[derive(Clone, Default)]
pub struct TripleUp;

impl Item for TripleUp {
    fn name(&self) -> String {
        "Triple-Up".to_string()
    }

    fn cost(&self) -> usize { 350 }

    fn description(&self, player: &mut Player) -> String {
        format!("Generate Triple Gold on Killing Enemies")
    }

    fn apply(&self, player: &mut Player) {
        player.gold_multi = 3.0;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { player.gold_multi == 2.0 }
}

inventory::submit!(ItemFactory(|| Box::new(TripleUp::default())));