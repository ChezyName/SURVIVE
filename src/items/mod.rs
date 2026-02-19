use bevy::prelude::*;
use crate::player::Player;

pub trait Item: Send + Sync {
    fn name(&self) -> String;
    fn cost(&self) -> usize;
    fn description(&self) -> String;
    fn apply(&self, player: &mut Player);
    fn clone_box(&self) -> Box<dyn Item>;
    fn is_unique(&self) -> bool; //if this item can only be obtained once
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