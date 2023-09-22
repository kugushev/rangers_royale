use bevy::prelude::*;
use crate::x_old::components::characters::animations::CharacterAnimationHandles;

pub(crate) fn build_characters_animations(app: &mut App) {
    app.add_system(switch_animations);
}

fn switch_animations(keyboard_input: Res<Input<KeyCode>>,
                     mut query: Query<(&CharacterAnimationHandles, &mut Handle<TextureAtlas>)>,
) {
    let run = keyboard_input.pressed(KeyCode::Space);

    for (handles, mut handle) in &mut query {
        *handle = if run {
            handles.run_down().clone_weak()
        }else {
            handles.idle_down().clone_weak()
        }
    }
}