use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalSounds {
    pub fire: Handle<AudioSource>,
    pub hurt: Handle<AudioSource>,
    pub lifeline: Handle<AudioSource>,
    pub missile: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,

    //shop
    pub shop_open: Handle<AudioSource>,
    pub shop_buy: Handle<AudioSource>,
}

impl FromWorld for GlobalSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            fire:       asset_server.load("audio/Fire.ogg"),
            hurt:       asset_server.load("audio/Hurt.ogg"),
            lifeline:   asset_server.load("audio/Lifeline.ogg"),
            missile:    asset_server.load("audio/Missile.ogg"),
            explosion:  asset_server.load("audio/Explosion.ogg"),

            shop_open:  asset_server.load("audio/Select.ogg"),
            shop_buy:   asset_server.load("audio/Coins.ogg"),
        }
    }
}

pub fn play_sfx(commands: &mut Commands, source: Handle<AudioSource>) { commands.spawn(AudioPlayer::new(source)); }
pub fn play_sfx_rand_pitch(commands: &mut Commands, source: Handle<AudioSource>) {
    let speed: f32 = rand::random_range(0.9..1.1);
    commands.spawn((AudioPlayer::new(source), PlaybackSettings { speed, ..PlaybackSettings::ONCE},));
}