use bevy::input::mouse::MouseButtonInput;
use bevy::{ecs::system::Resource, prelude::*};
use bevy_kira_audio::prelude::*;
use rand::seq::SliceRandom;
use rand::*;

use crate::assets::audio::SfxAssets;

pub(super) fn sfx_plugin(app: &mut bevy::prelude::App) {
    app.add_audio_channel::<SfxChannel>().add_system(
        play_sfx_on_evt::<MouseButtonInput>
            .in_base_set(CoreSet::PostUpdate)
            .run_if(resource_exists::<SfxAssets>()),
    );
}

#[derive(Resource)]
struct SfxChannel;

pub trait SfxEv {
    fn get_volume(&self) -> f64;
    fn get_sfx_handles(sfx: &SfxAssets) -> &[Handle<AudioSource>];
    fn skip(&self) -> bool {
        false
    }
}

impl SfxEv for MouseButtonInput {
    fn get_volume(&self) -> f64 {
        // 0.25.lerp(&1.0, &self.strength)
        0.2
    }

    fn get_sfx_handles(sfx: &SfxAssets) -> &[Handle<AudioSource>] {
        &sfx.click
    }

    fn skip(&self) -> bool {
        !self.state.is_pressed()
    }
}

fn play_sfx_on_evt<TEvt: Event + SfxEv>(
    mut ev_r: EventReader<TEvt>,
    audio: Res<AudioChannel<SfxChannel>>,
    sfx: Res<SfxAssets>,
) {
    let mut rand = thread_rng();

    for ev in ev_r.iter() {
        if ev.skip() {
            continue;
        }

        if let Some(sfx_handle) = TEvt::get_sfx_handles(&sfx).choose(&mut rand) {
            audio.play(sfx_handle.clone()).with_volume(ev.get_volume());
        }
    }
}