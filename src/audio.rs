use bevy::prelude::*;
use crate::AppState;

#[derive(Component)]
pub struct MusicTrack;

impl FromWorld for MusicPlayer {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            tracks: vec![
                (AppState::MainMenu,    asset_server.load("audio/Main Menu.ogg")),
                (AppState::InGame,      asset_server.load("audio/Gameplay.ogg")),
                (AppState::GameOver,    asset_server.load("audio/Game Over.ogg")),
                (AppState::Shop,        asset_server.load("audio/Shop Theme.ogg")),
            ],
        }
    }
}

#[derive(Resource)]
pub struct MusicPlayer {
    pub tracks: Vec<(AppState, Handle<AudioSource>)>,
}

pub struct LimitedSound {
    pub source: Handle<AudioSource>,
    pub max: u32,
    pub active: u32,
    pub id: &'static str,
}

impl LimitedSound {
    pub fn new(source: Handle<AudioSource>, max: u32, id: &'static str) -> Self {
        Self { source, active: 0, max, id }
    }
}

#[derive(Resource)]
pub struct GlobalSounds {
    pub fire: LimitedSound,
    pub hurt: LimitedSound,
    pub lifeline: LimitedSound,
    pub missile: LimitedSound,
    pub explosion: LimitedSound,
    pub enemy_kill: LimitedSound,

    //shop
    pub shop_open: LimitedSound,
    pub shop_buy: LimitedSound,
}

impl FromWorld for GlobalSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            fire:       LimitedSound::new(asset_server.load("audio/Fire.ogg"),       15,     "fire"),
            hurt:       LimitedSound::new(asset_server.load("audio/Hurt.ogg"),       10,     "hurt"),
            enemy_kill: LimitedSound::new(asset_server.load("audio/EnemyHurt.ogg"),  10,     "enemy_hurt"),
            lifeline:   LimitedSound::new(asset_server.load("audio/Lifeline.ogg"),   1,      "lifeline"),
            missile:    LimitedSound::new(asset_server.load("audio/Missile.ogg"),    1,      "missile"),
            explosion:  LimitedSound::new(asset_server.load("audio/Explosion.ogg"),  15,     "explosion"),
            shop_open:  LimitedSound::new(asset_server.load("audio/Select.ogg"),     5,      "shop_open"),
            shop_buy:   LimitedSound::new(asset_server.load("audio/Coins.ogg"),      15,     "shop_buy"),
        }
    }
}

//For sound cleanup
#[derive(Component)]
pub struct SoundMarker(pub &'static str);

pub fn play_sfx_rand_pitch(commands: &mut Commands, sound: &mut LimitedSound) {
    if sound.active < sound.max {
        sound.active += 1;
        let speed: f32 = rand::random_range(0.9..1.1);
        commands.spawn((
            AudioPlayer::new(sound.source.clone()),
            PlaybackSettings { speed, ..PlaybackSettings::ONCE },
            SoundMarker(sound.id),
        ));
    }
}

pub fn cleanup(
    mut commands: Commands,
    query: Query<(Entity, &AudioSink, &SoundMarker)>,
    mut sounds: ResMut<GlobalSounds>,
) {
    for (entity, sink, marker) in &query {
        if sink.empty() {
            commands.entity(entity).despawn();
            let GlobalSounds { fire, hurt, lifeline, missile, explosion, enemy_kill, shop_open, shop_buy } = sounds.as_mut();
            let all: [&mut LimitedSound; 8] = [fire, hurt, lifeline, missile, explosion, enemy_kill, shop_open, shop_buy];

            for sound in all {
                if sound.id == marker.0 {
                    sound.active = sound.active.saturating_sub(1);
                    break;
                }
            }
        }
    }
}

//muffles audio when paused
pub fn handle_pause_audio(
    keys: Res<ButtonInput<KeyCode>>,
    mut music_query: Query<&mut AudioSink, With<MusicTrack>>,
    mut sfx_query: Query<&mut AudioSink, Without<MusicTrack>>,
) {
    let paused = keys.pressed(KeyCode::Tab);
    
    for mut sink in music_query.iter_mut() {
        sink.set_volume(if paused { bevy::audio::Volume::Linear(0.25) } else { bevy::audio::Volume::Linear(0.5) });
        sink.set_speed(if paused { 0.85 } else { 1.0 });
    }

    for mut sink in sfx_query.iter_mut() {
        sink.set_volume(if paused { bevy::audio::Volume::Linear(0.5) } else { bevy::audio::Volume::Linear(1.0) });
    }
}

pub fn change_music(
    mut commands: Commands,
    music_query: Query<Entity, With<MusicTrack>>,
    music_player: Res<MusicPlayer>,
    state: Res<State<AppState>>,
) {
    for entity in &music_query {
        commands.entity(entity).despawn();
    }

    let current = state.get();
    if let Some((_, source)) = music_player.tracks.iter().find(|(s, _)| s == current) {
        commands.spawn((
            AudioPlayer::new(source.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::Linear(0.5),
                ..default()
            },
            MusicTrack,
        ));
    }
}