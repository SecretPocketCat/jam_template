use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

use crate::AppState;

pub mod audio;
pub mod fonts;
pub mod textures;
mod window_icon;

use self::window_icon::set_window_icon;

pub fn assets_plugin(app: &mut App) {
    app.add_startup_system(set_window_icon)
        .add_loading_state(LoadingState::new(AppState::Splash).continue_to_state(AppState::Loading))
        .add_collection_to_loading_state::<_, textures::SplashScreenTextureAssets>(AppState::Splash)
        .add_loading_state(LoadingState::new(AppState::Loading))
        .add_collection_to_loading_state::<_, fonts::FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, audio::SfxAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, audio::AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, textures::TextureAssets>(AppState::Loading)
        .add_startup_system(audio::setup_sfx_assets)
        .add_plugin(ProgressPlugin::new(AppState::Loading))
        .add_system(print_progress.in_set(OnUpdate(AppState::Loading)));
}

fn add_dynamic_assets(
    dynamic_assets: &mut DynamicAssets,
    key: &str,
    file_prefix: &str,
    file_ext: &str,
    range: RangeInclusive<usize>,
) {
    dynamic_assets.register_asset(
        key,
        Box::new(StandardDynamicAsset::Files {
            paths: range
                .map(|i| format!("{file_prefix}{i}.{file_ext}"))
                .collect(),
        }),
    );
}

fn print_progress(progress: Res<ProgressCounter>, mut last_done: Local<u32>) {
    let progress = progress.progress();
    warn!("Progress: {:?}", progress);

    if progress.done > *last_done {
        *last_done = progress.done;
        warn!("Changed progress: {:?}", progress);
    }
}
