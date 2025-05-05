use crate::helpers::button::ChangedButtonsFilter;
use bevy::{prelude::*, ui::Interaction};
use bevy_tweening::{lens::TransformScaleLens, Animator, Tween};
use std::time::Duration;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_anim_system);
    }
}

fn button_anim_system(
    mut interaction_query: Query<
        (&Interaction, &mut Animator<Transform>, &Transform),
        ChangedButtonsFilter,
    >,
) {
    for (interaction, mut animator, transform) in &mut interaction_query {
        let start_scale = transform.scale;
        match *interaction {
            Interaction::Pressed => {
                animator.set_tweenable(Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: start_scale,
                        end: Vec3::splat(0.95),
                    },
                ));
            }
            Interaction::Hovered => {
                animator.set_tweenable(Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: start_scale,
                        end: Vec3::splat(1.05),
                    },
                ));
            }
            Interaction::None => {
                animator.set_tweenable(Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: start_scale,
                        end: Vec3::ONE,
                    },
                ));
            }
        }
    }
}
