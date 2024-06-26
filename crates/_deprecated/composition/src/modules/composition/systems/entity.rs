use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use glam::{Mat3, Vec2};

use crate::modules::{
    composition::events::{EntityDeleted, EntityMoved, EntitySetPosition},
    node::components::mixins::RelativeTransformMixin,
};

// https://bevy-cheatbook.github.io/fundamentals/hierarchy.html#despawning-child-entities
// https://github.com/bevyengine/bevy/issues/5584
pub fn handle_entity_deleted(mut commands: Commands, mut event_reader: EventReader<EntityDeleted>) {
    for event in event_reader.read() {
        commands.entity(event.entity).despawn_recursive();
    }
}

pub fn handle_entity_moved(
    mut event_reader: EventReader<EntityMoved>,
    mut query: Query<&mut RelativeTransformMixin>,
) {
    for event in event_reader.read() {
        let EntityMoved { entity, dx, dy } = event;
        if let Ok(mut mixin) = query.get_mut(*entity) {
            let translation = Mat3::from_translation(Vec2::new(*dx, *dy));
            mixin.0 = mixin.0 * translation;
        }
    }
}

pub fn handle_entity_set_position(
    mut event_reader: EventReader<EntitySetPosition>,
    mut query: Query<&mut RelativeTransformMixin>,
) {
    for event in event_reader.read() {
        let EntitySetPosition { entity, x, y } = event;
        if let Ok(mut mixin) = query.get_mut(*entity) {
            mixin.0.col_mut(2).x = *x;
            mixin.0.col_mut(2).y = *y;
        }
    }
}

#[cfg(feature = "interactive")]
pub mod interactive {
    use std::collections::HashSet;

    use bevy_ecs::{
        entity::Entity,
        query::Changed,
        system::{Commands, Query},
    };
    use bevy_hierarchy::Parent;

    use crate::modules::node::components::mixins::{
        AbsoluteTransformMixin, RelativeTransformMixin,
    };

    pub fn calculate_absolute_transform(
        mut commands: Commands,
        query_children: Query<
            (Entity, &Parent, &RelativeTransformMixin),
            Changed<RelativeTransformMixin>,
        >,
        query_parents: Query<(Entity, &AbsoluteTransformMixin), Changed<AbsoluteTransformMixin>>,
        query_all_children: Query<(Entity, &Parent)>,
    ) {
        let mut children_to_update = HashSet::new();

        // Add children with changed relative transforms
        for (child_entity, _, _) in query_children.iter() {
            children_to_update.insert(child_entity);
        }

        // Add all children of parents with changed absolute transforms
        for (parent_entity, _) in query_parents.iter() {
            for (child_entity, parent) in query_all_children.iter() {
                if parent.get() == parent_entity {
                    children_to_update.insert(child_entity);
                }
            }
        }

        // Apply updates
        for child_entity in children_to_update {
            if let Ok((_, parent, child_transform)) = query_children.get(child_entity) {
                if let Ok(parent_transform) =
                    query_parents.get_component::<AbsoluteTransformMixin>(parent.get())
                {
                    let absolute_transform = parent_transform.0 * child_transform.0;
                    commands
                        .entity(child_entity)
                        .insert(AbsoluteTransformMixin(absolute_transform));
                }
            }
        }
    }
}
