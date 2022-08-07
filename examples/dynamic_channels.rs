use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(play)
        .add_system(pan)
        .run()
}

fn play(asset_server: Res<AssetServer>, mut audio: ResMut<DynamicAudioChannels>) {
    audio
        .create_channel("test")
        .play_looped(asset_server.load("sounds/loop.ogg"));
}

fn pan(time: Res<Time>, audio: Res<DynamicAudioChannels>) {
    let t = time.seconds_since_startup() as f32;
    audio.channel("test").set_panning(t.sin() * 0.5 + 0.5)
}
