use bevy::{prelude::*, ui::Interaction};
use bevy_tweening::{Lens, Tween, TweenAnim};
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiTransformScaleLens {
    pub start: Vec2,
    pub end: Vec2,
}

impl Lens<UiTransform> for UiTransformScaleLens {
    fn lerp(&mut self, mut target: Mut<UiTransform>, ratio: f32) {
        target.scale = self.start + (self.end - self.start) * ratio;
    }
}

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_anim_system);
    }
}

#[allow(clippy::type_complexity)]
fn button_anim_system(
    mut cmd: Commands,
    mut interaction_query: Query<
        (&Interaction, Entity, &UiTransform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, e, transform) in &mut interaction_query {
        let start_scale = transform.scale;
        let Ok(mut ent) = cmd.get_entity(e) else {
            continue;
        };
        match *interaction {
            Interaction::Pressed => ent.insert(TweenAnim::new(Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(200),
                UiTransformScaleLens {
                    start: start_scale,
                    end: Vec2::splat(0.95),
                },
            ))),
            Interaction::Hovered => ent.insert(TweenAnim::new(Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(200),
                UiTransformScaleLens {
                    start: start_scale,
                    end: Vec2::splat(1.05),
                },
            ))),
            Interaction::None => ent.insert(TweenAnim::new(Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(200),
                UiTransformScaleLens {
                    start: start_scale,
                    end: Vec2::ONE,
                },
            ))),
        };
    }
}
