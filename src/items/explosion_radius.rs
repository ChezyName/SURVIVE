use super::{Item, ItemFactory};
use crate::{config, player::Player};

const EXPLOSION_RADIUS: f32 = 0.1;

#[derive(Clone, Default)]
pub struct Explosion;

impl Item for Explosion {
    fn name(&self) -> String {
        "Explosive++".to_string()
    }

    fn cost(&self) -> usize { 200 }

    fn description(&self, player: &mut Player) -> String {
        format!("Increases Explosion Radius by {}%", config::format(EXPLOSION_RADIUS * 100.0))
    }

    fn apply(&self, player: &mut Player) {
        player.explosive_radius += EXPLOSION_RADIUS;
    }

    fn is_unique(&self) -> bool { false }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn can_buy(&self, player: &mut Player) -> bool { 
        player.explosive_bullets && player.explosive_radius <= 1.5
    }
}

inventory::submit!(ItemFactory(|| Box::new(Explosion::default())));