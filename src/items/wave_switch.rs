use super::{Item, ItemFactory};
use crate::{player::Player};

#[derive(Clone, Default)]
pub struct Explosion;

impl Item for Explosion {
    fn name(&self) -> String { "Wave X Switch".to_string() }

    fn cost(&self) -> usize { 500 }

    fn description(&self, _player: &mut Player) -> String {
        format!("Allows Missiles to target enemies hit by Waves.")
    }

    fn apply(&self, player: &mut Player) { player.wave_missile = true; }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { player.wave_time != -1.0 && player.missiles > 0 }
}

inventory::submit!(ItemFactory(|| Box::new(Explosion::default())));