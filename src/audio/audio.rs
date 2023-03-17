use crate::{assets::audio::AudioAssets, input::actions::Actions};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub(super) struct FlyingAudio(Handle<AudioInstance>);

pub(super) fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

pub(super) fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
