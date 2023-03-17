use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    // todo: macro?
    #[asset(
        paths("audio/sfx/ui/click1.ogg", "audio/sfx/ui/click2.ogg"),
        collection(typed)
    )]
    pub click: Vec<Handle<AudioSource>>,
}
