use bevy::prelude::*;

pub(super) fn build_disability(app: &mut App) {

}

pub enum Disability {
    Stun(Timer),
}