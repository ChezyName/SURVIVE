use super::{Item, ItemFactory};
use crate::{player::Player};

#[derive(Clone, Default)]
pub struct Explosion;

impl Item for Explosion {
    fn name(&self) -> String { "Explosive X Switch".to_string() }

    fn cost(&self) -> usize { 750 }

    fn description(&self, player: &mut Player) -> String {
        format!("Allows Missiles to target enemies hit by Explosions.")
    }

    fn apply(&self, player: &mut Player) {
        player.missile_explosion = true;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { player.explosive_bullets && player.missiles > 0 }
}

inventory::submit!(ItemFactory(|| Box::new(Explosion::default())));