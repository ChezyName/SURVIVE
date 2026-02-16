use bevy::prelude::*;
use crate::player::Player;

pub trait ItemLogic: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn upgrade(&mut self);
    fn can_upgrade(&self) -> bool;
    fn apply(&self, player: &mut Player);
    fn clone_item(&self) -> Box<dyn ItemLogic>;
}