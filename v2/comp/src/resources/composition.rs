use bevy_ecs::{entity::Entity, system::Resource};
use dyn_comp_types::shared::{Size, Viewport};

#[derive(Resource, Debug)]
pub struct CompositionRes {
    pub root_nodes: Vec<Entity>,
    pub viewport: Viewport,
    pub size: Size,
}
