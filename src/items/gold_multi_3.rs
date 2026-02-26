use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

#[derive(Clone, Default)]
pub struct QuadUp;

impl Item for QuadUp {
    fn name(&self) -> String {
        "4".to_string()
    }

    fn cost(&self) -> usize { 450 }

    fn description(&self, player: &mut Player) -> String {
        format!("Generate FOUR TIMES Gold on Killing Enemies. FANTASTIC")
    }

    fn apply(&self, player: &mut Player) {
        player.gold_multi = 4.0;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { player.gold_multi == 3.0 }
}

inventory::submit!(ItemFactory(|| Box::new(QuadUp::default())));