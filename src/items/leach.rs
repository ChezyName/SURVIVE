use super::{Item, ItemFactory};
use crate::{player::Player};
use crate::config::format;

//adds one pellet per
const LIFE_STEAL_PERCENT_INCREASE: f32 = 2.5;
const COST: usize = 25;

#[derive(Clone, Default)]
pub struct LifeSteal;

impl Item for LifeSteal {
    fn name(&self) -> String {
        "Life Steal".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Increases lifesteal by {}%", format(LIFE_STEAL_PERCENT_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        player.life_steal = (player.life_steal + LIFE_STEAL_PERCENT_INCREASE).clamp(0.0, 100.0);
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool {
        return player.life_steal < 100.0
    }
}

inventory::submit!(ItemFactory(|| Box::new(LifeSteal::default())));