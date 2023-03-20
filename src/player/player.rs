use crate::{
    assets::textures::TextureAssets,
    input::actions::PlayerAction,
    time::time::{ScaledTime, ScaledTimeDelta},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

// todo: bind gamepads?
pub(super) fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(InputManagerBundle::<PlayerAction> {
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), PlayerAction::Move)
                .insert(VirtualDPad::wasd(), PlayerAction::Move)
                .insert(VirtualDPad::arrow_keys(), PlayerAction::Move)
                .build(),
            ..default()
        });
}

pub(super) fn move_player(
    time: ScaledTime,
    mut player_query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>,
) {
    let speed = 150.;

    for (mut player_transform, actions) in &mut player_query {
        if let Some(movement) = actions.clamped_axis_pair(PlayerAction::Move) && movement.xy() != Vec2::ZERO {
            player_transform.translation += movement.xy().extend(0.) *  speed * time.delta_seconds();
        }
    }
}
