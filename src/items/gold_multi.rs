use super::{Item, ItemFactory};
use crate::player::Player;

#[derive(Clone, Default)]
pub struct DoubleUp;

impl Item for DoubleUp {
    fn name(&self) -> String { "Double-Up".to_string() }

    fn cost(&self) -> usize { 250 }

    fn description(&self, _player: &mut Player) -> String {
        format!("Generate Double Gold on Killing Enemies")
    }

    fn apply(&self, player: &mut Player) {
        player.gold_multi = 2.0;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, _player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(DoubleUp::default())));