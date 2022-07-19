use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(play)
        .run()
}

fn play(asset_server: Res<AssetServer>, mut audio: ResMut<DynamicAudioChannels>) {
    audio
        .create_channel("test")
        .play_looped(asset_server.load("sounds/loop.ogg"));
}
