use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const GOLD_PER_SECOND: f32 = 3.5;

#[derive(Clone, Default)]
pub struct PassiveIncome;

impl Item for PassiveIncome {
    fn name(&self) -> String { "Pa$$iv3 1nc0m3".to_string() }

    fn cost(&self) -> usize { 1000 }

    fn description(&self, player: &mut Player) -> String {
        format!("Generate an Extra {} Gold per Second", GOLD_PER_SECOND)
    }

    fn apply(&self, player: &mut Player) {
        player.gold_per_second += GOLD_PER_SECOND;
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(PassiveIncome::default())));