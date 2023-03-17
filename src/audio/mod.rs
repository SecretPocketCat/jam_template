use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use seldom_fn_plugin::FnPluginExt;

use self::audio::{control_flying_sound, start_audio};
use crate::GameState;

mod audio;
pub mod sfx;

pub fn audio_plugin(app: &mut App) {
    app.add_plugin(AudioPlugin)
        .fn_plugin(sfx::sfx_plugin)
        .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
        .add_system(control_flying_sound.in_set(OnUpdate(GameState::Playing)));
}
