use super::{Item, ItemFactory};
use crate::{player::Player};

//adds one pellet per
const BULLET_SIZE_PERCENT_INCREASE: f32 = 15.0;
const COST: usize = 100;

#[derive(Clone, Default)]
pub struct BulletSize;

impl Item for BulletSize {
    fn name(&self) -> String {
        "Caliber".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Increases bullet size by {:.2}%", BULLET_SIZE_PERCENT_INCREASE)
    }

    fn apply(&self, player: &mut Player) {
        player.bullet_size = player.bullet_size + BULLET_SIZE_PERCENT_INCREASE;
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