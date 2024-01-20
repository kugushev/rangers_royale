use bevy::prelude::*;
use crate::game::battle::characters::controllers::direct::build_direct;
use crate::game::battle::characters::controllers::indirect::build_indirect;

pub mod indirect;
pub mod direct;

pub(super) fn build_controllers(app: &mut App){
    build_indirect(app);
    build_direct(app);
}
