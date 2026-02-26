use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const CRIT_CHANGE_INCREASE: f32 = 10.0;
const COST: usize = 50;

#[derive(Clone, Default)]
pub struct Crit;

impl Item for Crit {
    fn name(&self) -> String {
        "Lady Luck".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self, player: &mut Player) -> String {
        format!("Increases Critical Strike Change by {}%", format(CRIT_CHANGE_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        player.crit_percent = (player.crit_percent + CRIT_CHANGE_INCREASE).clamp(0.0, 100.0);
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { return player.crit_percent < 100.0; }
}

inventory::submit!(ItemFactory(|| Box::new(Crit::default())));