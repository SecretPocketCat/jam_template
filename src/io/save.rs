use bevy::{prelude::*, window::WindowCloseRequested};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::{state::GameState, GAME_NAME};

const SETTINGS_KEY: &str = "game_save";

pub(super) fn save_plugin(app: &mut App) {
    app.insert_resource(PkvStore::new("SecretPocketCat", GAME_NAME))
        .add_system(
            save_game
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_changed::<GameState>().or_else(on_event::<WindowCloseRequested>())),
        )
        .add_startup_system(load_game);
}

#[derive(Serialize, Deserialize, Resource, Clone)]
pub struct Volume {
    master: f32,
    sfx: f32,
    music: f32,
    muted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GameSettings {
    volume: Volume,
}

fn save_game(mut pkv: ResMut<PkvStore>, volume_set: Res<Volume>) {
    pkv.set(
        SETTINGS_KEY,
        &GameSettings {
            volume: volume_set.clone(),
        },
    )
    .expect("failed to store settings");
}

fn load_game(pkv: Res<PkvStore>, mut cmd: Commands) {
    let settings = pkv
        .get::<GameSettings>(SETTINGS_KEY)
        .unwrap_or_else(|_| GameSettings {
            volume: Volume {
                master: 0.5,
                sfx: 0.5,
                music: 0.5,
                muted: false,
            },
        });

    cmd.insert_resource(settings.volume);
}
