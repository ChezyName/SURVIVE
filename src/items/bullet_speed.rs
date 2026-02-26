use super::{Item, ItemFactory};
use crate::{player::Player};
use crate::config::format;

//adds one pellet per
const BULLET_SPEED_INCREASE: f32 = 150.0;
const COST: usize = 100;

#[derive(Clone, Default)]
pub struct BulletSpeed;

impl Item for BulletSpeed {
    fn name(&self) -> String {
        "Railgun".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self, player: &mut Player) -> String {
        format!("Increases Bullet Speed by {} m/s", format(BULLET_SPEED_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        player.bullet_speed += BULLET_SPEED_INCREASE;

    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(BulletSpeed::default())));