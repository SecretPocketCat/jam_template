use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SplashScreenTextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
}
