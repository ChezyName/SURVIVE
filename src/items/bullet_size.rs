use super::{Item, ItemFactory};
use crate::{player::Player};
use crate::config::format;

//adds one pellet per
const BULLET_SIZE_PERCENT_INCREASE: f32 = 15.0;
const BULLET_SIZE_PIERCE: usize = 1;
const COST: usize = 100;

#[derive(Clone, Default)]
pub struct BulletSize;

impl Item for BulletSize {
    fn name(&self) -> String {
        "Caliber".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Increases bullet size by {}% and allows you to pierce {} additional target", format(BULLET_SIZE_PERCENT_INCREASE), BULLET_SIZE_PIERCE)
    }

    fn apply(&self, player: &mut Player) {
        player.bullet_size = player.bullet_size + BULLET_SIZE_PERCENT_INCREASE;
        player.bullet_pierce += BULLET_SIZE_PIERCE;

    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool {
        return true;
    }
}

inventory::submit!(ItemFactory(|| Box::new(BulletSize::default())));