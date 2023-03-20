use bevy::prelude::*;

use self::time::TimeScale;

pub mod time;

pub fn time_plugin(app: &mut App) {
    app.insert_resource(TimeScale(1.));
}
