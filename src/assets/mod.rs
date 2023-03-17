use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub mod audio;
pub mod fonts;
pub mod textures;
mod window_icon;

use self::window_icon::set_window_icon;

pub fn assets_plugin(app: &mut App) {
    app.add_startup_system(set_window_icon)
        .add_startup_system(audio::setup_sfx_assets)
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu))
        .add_collection_to_loading_state::<_, fonts::FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, audio::SfxAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, audio::AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, textures::TextureAssets>(GameState::Loading);
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
