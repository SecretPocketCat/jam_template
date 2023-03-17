use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use self::audio::{control_flying_sound, start_audio};
use crate::GameState;

mod audio;

pub fn audio_plugin(app: &mut App) {
    app.add_plugin(AudioPlugin)
        .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
        .add_system(control_flying_sound.in_set(OnUpdate(GameState::Playing)));
}
