use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_tweening::Animator;
use iyes_progress::ProgressCounter;

use crate::{
    animation::{delay_tween, get_fade_in_tween},
    state::AppState,
    state_machine::{command::CommandStateMachineExtensions, trigger::StateTrigger, StateMachine},
};

pub fn splash_plugin(app: &mut App) {
    app.add_system(setup_splash.in_schedule(OnEnter(AppState::Loading)))
        .add_system(run_splash_sequence.in_set(OnUpdate(AppState::Loading)));
}

#[derive(AssetCollection, Resource)]
pub struct SplashScreenTextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SplashSequence {
    StartFadeIn,
    Fading,
    Loading,
    StartFadeOut,
    Done,
}

fn setup_splash(mut cmd: Commands) {
    cmd.spawn(StateMachine::new_sequence(
        SplashSequence::StartFadeIn,
        vec![
            (StateTrigger::SingleFrame, SplashSequence::Fading),
            (StateTrigger::TweenDone, SplashSequence::Loading),
            (
                StateTrigger::TweenDone.and(
                    StateTrigger::Timer(Duration::from_secs_f32(3.))
                        .and(StateTrigger::DoneInserted),
                ),
                SplashSequence::StartFadeOut,
            ),
            (StateTrigger::SingleFrame, SplashSequence::Fading),
            (StateTrigger::TweenDone, SplashSequence::Done),
        ],
    ));
}

fn run_splash_sequence(
    mut cmd: Commands,
    seq_q: Query<(Entity, &StateMachine<SplashSequence>)>,
    progress: Res<ProgressCounter>,
    splash: Res<SplashScreenTextureAssets>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (sm_e, sm) in seq_q.iter() {
        match sm.state() {
            SplashSequence::StartFadeIn => {
                cmd.entity(sm_e).insert((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::NONE,
                            ..default()
                        },
                        texture: splash.bevy.clone(),
                        ..default()
                    },
                    Animator::new(delay_tween(get_fade_in_tween(300, None), 1000)),
                ));
            }
            SplashSequence::Fading => {}
            SplashSequence::Loading => {
                let progress = progress.progress();
                if progress.total > 0 && progress.done == progress.total {
                    cmd.state_machine_done::<SplashSequence>(sm_e);
                }
            }
            SplashSequence::StartFadeOut => {
                cmd.entity(sm_e)
                    .insert(Animator::new(get_fade_in_tween(300, None)));
            }
            SplashSequence::Done => {
                next_app_state.set(AppState::Menu);
            }
        }
    }
}
