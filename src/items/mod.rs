use bevy::prelude::*;
use crate::player::Player;

pub trait Item: Send + Sync {
    fn name(&self) -> String;
    fn cost(&self) -> usize;
    fn description(&self, player: &mut Player) -> String;
    fn apply(&self, player: &mut Player);
    fn clone_box(&self) -> Box<dyn Item>;
    fn is_unique(&self) -> bool; //if this item can only be obtained once
    fn can_buy(&self, player: &mut Player) -> bool; //if this item can be bought as of right now
}

impl Clone for Box<dyn Item> {
    fn clone(&self) -> Box<dyn Item> {
        self.clone_box()
    }
}

pub struct ItemFactory(pub fn() -> Box<dyn Item>);
inventory::collect!(ItemFactory);

//Items
pub mod rapid_fire;
pub mod max_health;
pub mod damage;
pub mod shotgun;
pub mod leach;
pub mod bullet_size;
pub mod crit;
pub mod lifeline;
pub mod accuracy;
pub mod explosion;
pub mod explosion_radius;
pub mod bullet_speed;
pub mod switch;
pub mod explosive_switch;
pub mod passive_income;
pub mod gold_multi;