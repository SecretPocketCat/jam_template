use bevy::prelude::*;
use web_sys;

pub fn save_plugin(app: &mut App) {
    app.add_system(save_game).add_startup_system(load_game);
}

const SAVE_KEY: &str = "game_save";

fn save_game(// mut lvl_evr: EventReader<LevelEv>, lvl: Res<CurrentLevel>
) {
    // for ev in lvl_evr.iter() {
    //     if let LevelEv::LevelOut = ev {
    //         write_save(lvl.level_index);
    //     }
    // }
}

fn load_game() {
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
