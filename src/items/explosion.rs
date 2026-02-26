use super::{Item, ItemFactory};
use crate::{config, player::Player};

pub const EXPLOSION_DAMAGE_MULTI: f32 = 1.75; //Bullet Damage * This = Final Explosion Damage;

#[derive(Clone, Default)]
pub struct Explosion;

impl Item for Explosion {
    fn name(&self) -> String {
        "Explosive Rounds".to_string()
    }

    fn cost(&self) -> usize { 750 }

    fn description(&self, player: &mut Player) -> String {
        format!("Turns your bullets into explosive rounds.")
    }

    fn apply(&self, player: &mut Player) {
        player.explosive_bullets = true;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(Explosion::default())));