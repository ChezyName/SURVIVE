use super::{Item, ItemFactory};
use crate::{player::Player};
use std::sync::atomic::{AtomicUsize, Ordering};

//adds one pellet per
const COST: usize = 300;
const MAX_LEVEL: usize = 8;
pub const DAMAGE_REDUCTION_PER_BULLET: f32 = 15.0;  //15% damage for all bullets
pub const SPEED_MULTI: f32 = 175.0;                 //Extra speed for bullets
pub const SPEED_MIN: f32 = 350.0;                   //Extra speed for bullets in %
pub const SIZE_MULTI: f32 = 30.0;                   //Extra size for bulelts in %
pub const SPAWN_DELAY_MS: u64 = 150;                //Time between each spawn in ms

#[derive(Default)]
pub struct Switch {
    level: AtomicUsize,
}

impl Clone for Switch {
    fn clone(&self) -> Self {
        Self {
            level: AtomicUsize::new(self.level.load(Ordering::Relaxed)),
        }
    }
}

impl Item for Switch {
    fn name(&self) -> String {
        "Switch".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self) -> String {
        format!("Add 1 Extra Projectile to Missle. Auto fires homing missiles when you hit an enemy with a bullet.")
    }

    fn apply(&self, player: &mut Player) {
        self.level.fetch_add(1, Ordering::Relaxed);
        player.missiles += 1;
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool {
        let current_lvl = self.level.load(Ordering::Relaxed);
        return current_lvl < MAX_LEVEL
    }
}

inventory::submit!(ItemFactory(|| Box::new(Switch::default())));