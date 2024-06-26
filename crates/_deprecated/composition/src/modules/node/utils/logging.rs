use bevy_ecs::{entity::Entity, world::World};
use log::info;

pub fn log_entity_components(world: &World, entity: Entity) {
    let component_names = world
        .inspect_entity(entity)
        .iter()
        .map(|info| info.name())
        .collect::<Vec<_>>();

    if component_names.is_empty() {
        info!("Entity ({:?}) has no components.", entity);
    } else {
        info!(
            "Entity ({:?}) Components:\n - {}",
            entity,
            component_names.join("\n - ")
        );
    }
}
