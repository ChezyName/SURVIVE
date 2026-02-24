use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

#[derive(Clone, Default)]
pub struct Lifeline;

impl Item for Lifeline {
    fn name(&self) -> String {
        "Lifeline".to_string()
    }

    fn cost(&self) -> usize { 500 }

    fn description(&self) -> String {
        format!("Grants a one time revive upon taking fatal damage. (Healthbar becomes Gold)")
    }

    fn apply(&self, player: &mut Player) {
        player.has_lifeline = true;
    }

    fn is_unique(&self) -> bool { true }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { true }
}

inventory::submit!(ItemFactory(|| Box::new(Lifeline::default())));