use bevy::prelude::*;
use web_sys;

const SAVE_KEY: &str = "game_save";

pub(super) fn save_game(// mut lvl_evr: EventReader<LevelEv>, lvl: Res<CurrentLevel>
) {
    // for ev in lvl_evr.iter() {
    //     if let LevelEv::LevelOut = ev {
    //         write_save(lvl.level_index);
    //     }
    // }
}

pub(super) fn load_game() {
    // let lvl = read_save();
}

// todo: handle non-wasm, also error handling...
fn write_save(level: usize) {
    if cfg!(target_family = "wasm") {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.set_item(SAVE_KEY, &level.to_string()).unwrap();
    } else {
        todo!();
    }
}

// todo: handle non-wasm, also error handling...
fn read_save() -> usize {
    if cfg!(target_family = "wasm") {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match storage.get_item(SAVE_KEY).unwrap() {
            Some(val) => str::parse(&val).unwrap_or(0),
            None => 0,
        }
    } else {
        todo!();
    }
}
