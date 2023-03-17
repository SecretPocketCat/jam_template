use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

mod audio;
mod fonts;
mod textures;
mod window_icon;

pub use audio::AudioAssets;
pub use fonts::FontAssets;
pub use textures::TextureAssets;

use self::window_icon::set_window_icon;

pub fn assets_plugin(app: &mut App) {
    app.add_startup_system(set_window_icon)
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu))
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
}
