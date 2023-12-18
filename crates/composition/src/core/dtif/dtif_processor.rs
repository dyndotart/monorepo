use std::collections::HashMap;

use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::BuildWorldChildren;
use glam::Mat3;

use crate::core::modules::{
    composition::events::CoreInputEvent,
    node::components::{
        bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle, TextNodeBundle},
        mixins::{AbsoluteTransformMixin, ChildrenMixin, FillMixin, Paint, RelativeTransformMixin},
    },
};

use super::{DTIFNode, EntityId};

pub struct DTIFProcessor {
    /// Maps EntityId (eid) to actual spawned Bevy entity.
    eid_to_entity: HashMap<EntityId, Entity>,
}

impl DTIFProcessor {
    pub fn new() -> Self {
        DTIFProcessor {
            eid_to_entity: HashMap::new(),
        }
    }

    // =========================================================================
    // Node
    // =========================================================================

    /// Processes the root DTIF node and its children.
    pub fn process_root(
        &mut self,
        node_eid: EntityId,
        world: &mut World,
        nodes: &HashMap<EntityId, &DTIFNode>,
        paints: &HashMap<EntityId, &Paint>,
    ) -> Option<Entity> {
        self.process_node(node_eid, world, nodes, paints, true)
    }

    /// Processes a single DTIF node and its children.
    fn process_node(
        &mut self,
        node_eid: EntityId,
        world: &mut World,
        nodes: &HashMap<EntityId, &DTIFNode>,
        paints: &HashMap<EntityId, &Paint>,
        is_root: bool,
    ) -> Option<Entity> {
        nodes.get(&node_eid).map(|dtif_node| {
            // Spawn a new node entity from a DTIF node
            // and maintain a mapping from entity id to Bevy entity
            let node_entity = self.spawn_node(world, dtif_node);
            self.eid_to_entity.insert(node_eid, node_entity);

            // Set absolute transform for root node
            if is_root {
                let mut relative_transform: Option<Mat3> = None;
                if let Some(relative_transform_mixin) =
                    world.entity(node_entity).get::<RelativeTransformMixin>()
                {
                    relative_transform = Some(relative_transform_mixin.0.clone());
                }
                if let Some(relative_transform) = relative_transform {
                    world
                        .entity_mut(node_entity)
                        .insert(AbsoluteTransformMixin(relative_transform));
                }
            }

            self.process_fill(node_entity, world, paints, dtif_node);
            self.process_children(node_entity, world, nodes, paints, dtif_node);

            return node_entity;
        })
    }

    /// Processes the fill mixin of a DTIF node, if present.
    fn process_fill(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        paints: &HashMap<EntityId, &Paint>,
        dtif_node: &DTIFNode,
    ) {
        if let DTIFNode::Frame(FrameNodeBundle { fill_mixin, .. })
        | DTIFNode::Rectangle(RectangleNodeBundle { fill_mixin, .. })
        | DTIFNode::Text(TextNodeBundle { fill_mixin, .. }) = dtif_node
        {
            // Process paints and collect their Bevy entity ids
            let new_paints: Vec<Entity> = fill_mixin
                .paint_ids
                .iter()
                .filter_map(|paint_entity| {
                    let paint_eid = DTIFProcessor::entity_to_eid(paint_entity);
                    return self.process_paint(paint_eid, world, paints);
                })
                .collect();

            // Establish Bevy parent-child relationships for optimized interactions within the ECS.
            // For details, refer to: https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_paints.is_empty() {
                world.entity_mut(node_entity).push_children(&new_paints);
            }

            // Remove the temporary `ChildrenMixin` component.
            // Explanation:
            // After successfully establishing Bevy's internal parent-child relationships,
            // the `FillMixin` component, initially used to manage child entities
            // during the transition from the DTIF format, is no longer necessary.
            world.entity_mut(node_entity).remove::<FillMixin>();
        }
    }

    /// Processes the children of a DTIF node, if any.
    fn process_children(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        nodes: &HashMap<EntityId, &DTIFNode>,
        paints: &HashMap<EntityId, &Paint>,
        dtif_node: &DTIFNode,
    ) {
        if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
        | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children_mixin
                .0
                .iter()
                .filter_map(|child_entity| {
                    let child_eid = DTIFProcessor::entity_to_eid(child_entity);
                    return self.process_node(child_eid, world, nodes, paints, false);
                })
                .collect();

            // Establish Bevy parent-child relationships for optimized interactions within the ECS.
            // For details, refer to: https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_children.is_empty() {
                world.entity_mut(node_entity).push_children(&new_children);

                // Calculate & apply absolute transform to children based on the parents absolute transform
                let mut transform_updates = Vec::new();
                if let Some(parent_absolute_transform) =
                    world.entity(node_entity).get::<AbsoluteTransformMixin>()
                {
                    for &child in &new_children {
                        if let Some(child_relative_transform) =
                            world.entity(child).get::<RelativeTransformMixin>()
                        {
                            let child_absolute_transform =
                                parent_absolute_transform.0 * child_relative_transform.0;
                            transform_updates.push((child, child_absolute_transform));
                        }
                    }
                }
                for (child, absolute_transform) in transform_updates {
                    world
                        .entity_mut(child)
                        .insert(AbsoluteTransformMixin(absolute_transform));
                }
            }

            // Remove the temporary `ChildrenMixin` component.
            // Explanation:
            // After successfully establishing Bevy's internal parent-child relationships,
            // the `ChildrenMixin` component, initially used to manage child entities
            // during the transition from the DTIF format, is no longer necessary.
            world.entity_mut(node_entity).remove::<ChildrenMixin>();
        }
    }

    /// Spawns a DTIF node into the ECS world.
    fn spawn_node(&self, world: &mut World, node: &DTIFNode) -> Entity {
        match node {
            DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Text(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    // =========================================================================
    // Paint
    // =========================================================================

    /// Processes a single DTIF paint.
    pub fn process_paint(
        &mut self,
        paint_eid: EntityId,
        world: &mut World,
        paints: &HashMap<EntityId, &Paint>,
    ) -> Option<Entity> {
        paints.get(&paint_eid).map(|paint| {
            // Spawn paint
            let paint_entity = world.spawn((*paint).clone()).id();
            self.eid_to_entity.insert(paint_eid, paint_entity);

            return paint_entity;
        })
    }

    // =========================================================================
    // Event
    // =========================================================================

    /// Processes and sends the event into the ECS world.
    pub fn send_event_into_world(&self, event: CoreInputEvent, world: &mut World) {
        match event {
            CoreInputEvent::EntityMoved(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
            CoreInputEvent::EntitySetPosition(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
        }
    }

    // =========================================================================
    // Helper
    // =========================================================================

    /// Tries to find the actual spawned entity for a DTIF entity.
    fn find_entity(&self, dtif_entity: &Entity) -> Option<Entity> {
        let eid = DTIFProcessor::entity_to_eid(dtif_entity);
        self.eid_to_entity.get(&eid).cloned()
    }

    /// Converts an `Entity` to an `EntityId` (eid).
    #[inline]
    pub fn entity_to_eid(entity: &Entity) -> EntityId {
        EntityId(entity.to_bits())
    }
}
