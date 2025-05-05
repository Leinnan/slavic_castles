use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
pub struct BackgroundSprite;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<BackgroundSprite>();
    app.add_systems(Update, update_background_sprites.run_if(window_changed_or_component_added::<BackgroundSprite>));
}

pub fn window_changed_or_component_added<T: Component>(q: Query<(), Or<(Changed<Window>, Added<T>)>>) -> bool {
    !q.is_empty()
}

fn update_background_sprites(
    windows: Query<&Window>,
    mut query: Query<&mut Sprite, With<BackgroundSprite>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    for mut sprite in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }
}
