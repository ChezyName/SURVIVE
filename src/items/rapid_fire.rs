use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

const FIRE_RATE_INCREASE: f32 = 5.0;
const COST: usize = 80;

#[derive(Clone, Default)]
pub struct RapidFire;

impl Item for RapidFire {
    fn name(&self) -> String {
        "Rapid Fire".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Increases Fire Rate by {}", format(FIRE_RATE_INCREASE))
    }

    fn apply(&self, player: &mut Player) {
        let fire_rate = player.fire_rate + FIRE_RATE_INCREASE;
        player.fire_rate = fire_rate;
        player.fire_timer.set_duration(std::time::Duration::from_secs_f32(60.0/fire_rate));
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { return true; }
}

inventory::submit!(ItemFactory(|| Box::new(RapidFire::default())));