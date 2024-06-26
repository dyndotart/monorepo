use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_svg_render::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_entities: HashMap<Entity, Vec<MixinChange>>,
}
