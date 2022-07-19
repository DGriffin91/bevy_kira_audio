use crate::audio::{AudioCommand, PlayAudioCommandArgs, PlayAudioSettings};
use crate::{AudioSource, InstanceHandle, PlaybackState};
use bevy::asset::Handle;
use bevy::utils::HashMap;
use parking_lot::RwLock;
use std::any::TypeId;
use std::collections::VecDeque;

pub trait AudioControl {
    /// Play audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play(asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play(&self, audio_source: Handle<AudioSource>) -> InstanceHandle;

    /// Play looped audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play_looped(asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play_looped(&self, audio_source: Handle<AudioSource>) -> InstanceHandle;

    /// Play looped audio with an intro
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play_looped_with_intro(asset_server.load("intro.mp3"), asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play_looped_with_intro(
        &self,
        intro_audio_source: Handle<AudioSource>,
        looped_audio_source: Handle<AudioSource>,
    ) -> InstanceHandle;

    /// Stop all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.stop();
    /// }
    /// ```
    fn stop(&self);

    /// Pause all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.pause();
    /// }
    /// ```
    fn pause(&self);

    /// Resume all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.resume();
    /// }
    /// ```
    fn resume(&self);

    /// Set the volume
    ///
    /// The default value is 1.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_volume(0.5);
    /// }
    /// ```
    fn set_volume(&self, volume: f32);

    /// Set panning
    ///
    /// The default value is 0.5
    /// Values up to 1 pan to the right
    /// Values down to 0 pan to the left
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_panning(0.9);
    /// }
    /// ```
    fn set_panning(&self, panning: f32);

    /// Set playback rate
    ///
    /// The default value is 1
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_playback_rate(2.0);
    /// }
    /// ```
    fn set_playback_rate(&self, playback_rate: f32);

    /// Get state for a playback instance.
    fn state(&self, instance_handle: InstanceHandle) -> PlaybackState;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    Typed(TypeId),
    Dynamic(String),
}

#[derive(Default)]
pub struct DynamicAudioChannel {
    pub(crate) commands: RwLock<VecDeque<AudioCommand>>,
    pub(crate) states: HashMap<InstanceHandle, PlaybackState>,
}

impl AudioControl for DynamicAudioChannel {
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play(asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play(&self, audio_source: Handle<AudioSource>) -> InstanceHandle {
        let instance_handle = InstanceHandle::new();

        self.commands
            .write()
            .push_front(AudioCommand::Play(PlayAudioCommandArgs {
                settings: PlayAudioSettings {
                    source: audio_source,
                    intro_source: None,
                    looped: false,
                },
                instance_handle: instance_handle.clone(),
            }));

        instance_handle
    }

    /// Play looped audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play_looped(asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play_looped(&self, audio_source: Handle<AudioSource>) -> InstanceHandle {
        let instance_handle = InstanceHandle::new();

        self.commands
            .write()
            .push_front(AudioCommand::Play(PlayAudioCommandArgs {
                settings: PlayAudioSettings {
                    source: audio_source,
                    intro_source: None,
                    looped: true,
                },
                instance_handle: instance_handle.clone(),
            }));

        instance_handle
    }

    /// Play looped audio with an intro
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    ///     audio.play_looped_with_intro(asset_server.load("intro.mp3"), asset_server.load("audio.mp3"));
    /// }
    /// ```
    fn play_looped_with_intro(
        &self,
        intro_audio_source: Handle<AudioSource>,
        looped_audio_source: Handle<AudioSource>,
    ) -> InstanceHandle {
        let instance_handle = InstanceHandle::new();

        self.commands
            .write()
            .push_front(AudioCommand::Play(PlayAudioCommandArgs {
                settings: PlayAudioSettings {
                    source: looped_audio_source,
                    intro_source: Some(intro_audio_source),
                    looped: true,
                },
                instance_handle: instance_handle.clone(),
            }));

        instance_handle
    }

    /// Stop all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.stop();
    /// }
    /// ```
    fn stop(&self) {
        self.commands.write().push_front(AudioCommand::Stop);
    }

    /// Pause all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.pause();
    /// }
    /// ```
    fn pause(&self) {
        self.commands.write().push_front(AudioCommand::Pause);
    }

    /// Resume all audio
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.resume();
    /// }
    /// ```
    fn resume(&self) {
        self.commands.write().push_front(AudioCommand::Resume);
    }

    /// Set the volume
    ///
    /// The default value is 1.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_volume(0.5);
    /// }
    /// ```
    fn set_volume(&self, volume: f32) {
        self.commands
            .write()
            .push_front(AudioCommand::SetVolume(volume));
    }

    /// Set panning
    ///
    /// The default value is 0.5
    /// Values up to 1 pan to the right
    /// Values down to 0 pan to the left
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_panning(0.9);
    /// }
    /// ```
    fn set_panning(&self, panning: f32) {
        self.commands
            .write()
            .push_front(AudioCommand::SetPanning(panning));
    }

    /// Set playback rate
    ///
    /// The default value is 1
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_kira_audio::prelude::*;
    ///
    /// fn my_system(audio: Res<Audio>) {
    ///     audio.set_playback_rate(2.0);
    /// }
    /// ```
    fn set_playback_rate(&self, playback_rate: f32) {
        self.commands
            .write()
            .push_front(AudioCommand::SetPlaybackRate(playback_rate));
    }

    /// Get state for a playback instance.
    fn state(&self, instance_handle: InstanceHandle) -> PlaybackState {
        self.states
            .get(&instance_handle)
            .cloned()
            .unwrap_or_else(|| {
                self.commands
                    .read()
                    .iter()
                    .find(|command| match command {
                        AudioCommand::Play(PlayAudioCommandArgs {
                            instance_handle: handle,
                            settings: _,
                        }) => handle.id == instance_handle.id,
                        _ => false,
                    })
                    .map(|_| PlaybackState::Queued)
                    .unwrap_or(PlaybackState::Stopped)
            })
    }
}

#[derive(Default)]
pub struct DynamicAudioChannels {
    pub(crate) channels: HashMap<String, DynamicAudioChannel>,
}

impl DynamicAudioChannels {
    pub fn create_channel(&mut self, key: impl Into<String>) -> &DynamicAudioChannel {
        let key = key.into();
        self.channels
            .insert(key.clone(), DynamicAudioChannel::default());
        self.channels
            .get(&key)
            .expect("Failed to retrieve dynamic audio channel")
    }

    pub fn channel(&self, key: impl Into<String>) -> &DynamicAudioChannel {
        let key = key.into();
        assert!(
            self.channels.contains_key(&key),
            "Attempting to access dynamic audio channel '{:?}', which doesn't exist.",
            key
        );
        self.channels
            .get(&key)
            .expect("Failed to retrieve dynamic audio channel")
    }

    pub fn remove_channel(&mut self, key: impl Into<String>) {
        let key = key.into();
        self.channel(&key).stop();
        self.channels.remove(&key);
    }
}
