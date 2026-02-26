use super::{Item, ItemFactory};
use crate::{config, player::Player};
use std::sync::atomic::{AtomicUsize, Ordering};

//adds one pellet per
const COST: usize = 300;
const MAX_LEVEL: usize = 8;
pub const DAMAGE_REDUCTION_PER_BULLET: f32 = 15.0;  //15% damage for all bullets
pub const SPEED_MULTI: f32 = 175.0;                 //Extra speed for bullets
pub const SPEED_MIN: f32 = 350.0;                   //Extra speed for bullets in %
pub const SIZE_MULTI: f32 = 30.0;                   //Extra size for bulelts in %
pub const SPAWN_TIME_MS: u64 = 250;                 //Total time to spawn x Projectiles
pub const TURN_SPEED: [f32;2] = [2.5, 12.0];        //Speed of Turning Angle min-max (more accurate the closer the bullets are)
pub const MIN_MAX_DIST: [f32; 2] = [150.0, 1000.0]; //Distance min-max in which the bullets are more accurate
const DAMAGE_REDUCTION: f32 = 25.0;

#[derive(Default, Clone)]
pub struct Switch;

impl Item for Switch {
    fn name(&self) -> String {
        "Switch".to_string()
    }

    fn cost(&self) -> usize { COST }

    fn description(&self, player: &mut Player) -> String {
        format!("Add 1 Extra Projectile to Missle. Auto fires homing missiles when you hit an enemy with a bullet. Reduces pellet count by 1. Reduces damage by {}; Min={}", config::format(DAMAGE_REDUCTION), config::format(config::BULLET_DAMAGE))
    }

    fn apply(&self, player: &mut Player) {
        player.missiles += 1;
        player.pellets = (player.pellets - 1).max(1);
        player.damage = (player.damage - DAMAGE_REDUCTION).max(config::BULLET_DAMAGE)
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool {
        return player.missiles < MAX_LEVEL
    }
}

inventory::submit!(ItemFactory(|| Box::new(Switch::default())));