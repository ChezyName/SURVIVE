use super::ItemLogic;
use crate::player::Player;
use crate::config;

const MAX_LEVEL: u32 = 5;
const FIRE_RATE_PER_LEVEL: f32 = 600.0;

#[derive(Clone)]
pub struct RapidFire {
    pub level: u32,
}

impl Default for RapidFire {
    fn default() -> Self {
        Self {
            level: 1,
        }
    }
}

impl ItemLogic for RapidFire {
    fn name(&self) -> String {
        format!("Rapid Fire (Lvl {})", self.level)
    }

    fn can_upgrade(&self) -> bool {
        self.level < MAX_LEVEL
    }

    fn upgrade(&mut self) {
        if self.can_upgrade() {
            self.level += 1;
        }
    }

    fn apply(&self, player: &mut Player) {
        let fire_rate = config::PLAYER_FIRE_RATE + (self.level as f32 * FIRE_RATE_PER_LEVEL);
        player.fire_rate = fire_rate;
        player.fire_timer = Timer::from_seconds(60.0/fire_rate, TimerMode::Once);
    }

    fn clone_item(&self) -> Box<dyn ItemLogic> {
        Box::new(self.clone())
    }
}