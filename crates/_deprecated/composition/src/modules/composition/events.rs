use bevy_ecs::{entity::Entity, event::Event, world::World};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{events::input_event::InputEvent, modules::node::components::bundles::NodeBundle};

use super::resources::composition::ViewBox;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum CoreInputEvent {
    EntityMoved(EntityMoved),
    EntitySetPosition(EntitySetPosition),
    EntityDeleted(EntityDeleted),
    NodeCreated(NodeCreated),
    CompositionResized(CompositionResized),
    CompositionViewBoxChanged(CompositionViewBoxChanged),
}

impl InputEvent for CoreInputEvent {
    fn send_to_ecs(self, world: &mut World) {
        match self {
            // Composition Events
            CoreInputEvent::CompositionResized(event) => {
                world.send_event(event);
            }
            CoreInputEvent::CompositionViewBoxChanged(event) => {
                world.send_event(event);
            }

            // Node Events
            CoreInputEvent::NodeCreated(event) => {
                world.send_event(event);
            }
            CoreInputEvent::EntityDeleted(event) => {
                world.send_event(event);
            }

            // Entity Events
            CoreInputEvent::EntityMoved(event) => {
                world.send_event(event);
            }
            CoreInputEvent::EntitySetPosition(event) => {
                world.send_event(event);
            }
        }
    }
}

// =============================================================================
// Composition Events
// =============================================================================

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CompositionResized {
    pub width: f32,
    pub height: f32,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompositionViewBoxChanged {
    pub view_box: ViewBox,
}

// =============================================================================
// Node Events
// =============================================================================

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeCreated {
    pub parent_entity: Option<Entity>,
    pub node: NodeBundle,
}

// =============================================================================
// Entity Events
// =============================================================================

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct EntityMoved {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct EntitySetPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct EntityDeleted {
    pub entity: Entity,
}
