use bevy::prelude::*;
use crate::AppState;

#[derive(Component)]
pub struct MusicTrack;

#[derive(Resource, Default)]
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

#[derive(Resource, Default)]
pub struct GlobalSounds {
    pub fire: Option<LimitedSound>,
    pub hurt: Option<LimitedSound>,
    pub lifeline: Option<LimitedSound>,
    pub missile: Option<LimitedSound>,
    pub explosion: Option<LimitedSound>,
    pub enemy_kill: Option<LimitedSound>,

    //shop
    pub shop_open: Option<LimitedSound>,
    pub shop_buy: Option<LimitedSound>,
}

pub fn load_sounds(
    mut sounds: ResMut<GlobalSounds>,
    mut music: ResMut<MusicPlayer>,
    asset_server: Res<AssetServer>,

    mut commands: Commands,
    music_query: Query<Entity, With<MusicTrack>>,
    state: Res<State<AppState>>,
) {
    sounds.fire       = Some(LimitedSound::new(asset_server.load("embedded://audio/Fire.ogg"),       15, "fire"));
    sounds.hurt       = Some(LimitedSound::new(asset_server.load("embedded://audio/Hurt.ogg"),       10, "hurt"));
    sounds.enemy_kill = Some(LimitedSound::new(asset_server.load("embedded://audio/EnemyHurt.ogg"),  10, "enemy_hurt"));
    sounds.lifeline   = Some(LimitedSound::new(asset_server.load("embedded://audio/Lifeline.ogg"),   1,  "lifeline"));
    sounds.missile    = Some(LimitedSound::new(asset_server.load("embedded://audio/Missile.ogg"),    1,  "missile"));
    sounds.explosion  = Some(LimitedSound::new(asset_server.load("embedded://audio/Explosion.ogg"),  15, "explosion"));
    sounds.shop_open  = Some(LimitedSound::new(asset_server.load("embedded://audio/Select.ogg"),     5,  "shop_open"));
    sounds.shop_buy   = Some(LimitedSound::new(asset_server.load("embedded://audio/Coins.ogg"),      15, "shop_buy"));
    music.tracks = vec![
        (AppState::MainMenu,    asset_server.load("embedded://audio/Main Menu.ogg")),
        (AppState::InGame,      asset_server.load("embedded://audio/Gameplay.ogg")),
        (AppState::GameOver,    asset_server.load("embedded://audio/Game Over.ogg")),
        (AppState::Shop,        asset_server.load("embedded://audio/Shop Theme.ogg")),
    ];

    //update music on load
    for entity in &music_query {
        commands.entity(entity).despawn();
    }

    let current = state.get();
    if let Some((_, source)) = music.tracks.iter().find(|(s, _)| s == current) {
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

//For sound cleanup
#[derive(Component)]
pub struct SoundMarker(pub &'static str);

pub fn play_sfx_rand_pitch(commands: &mut Commands, audio: &mut Option<LimitedSound>) {
    if let Some(sound) = audio {
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
            let all: [&mut Option<LimitedSound>; 8] = [fire, hurt, lifeline, missile, explosion, enemy_kill, shop_open, shop_buy];

            for audio in all {
                if let Some(sound) = audio {
                    if sound.id == marker.0 {
                        sound.active = sound.active.saturating_sub(1);
                        break;
                    }
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