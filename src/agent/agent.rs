use bevy::prelude::*;

use crate::time::time::{ScaledTime, ScaledTimeDelta};

#[derive(Component, Deref, Default)]
pub struct Direction(pub Vec2);

#[derive(Component, Deref, Default)]
pub struct Speed(pub f32);

pub(super) fn move_agent(
    mut velocity_q: Query<(&Direction, &Speed, &mut Transform)>,
    time: ScaledTime,
) {
    for (dir, speed, mut trans) in velocity_q.iter_mut() {
        trans.translation += dir.extend(0.) * speed.0 * time.scaled_delta_seconds();
    }
}
