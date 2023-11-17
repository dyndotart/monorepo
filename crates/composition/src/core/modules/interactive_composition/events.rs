use bevy_ecs::{entity::Entity, event::Event, world::World};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::events::input_event::InputEvent;

#[derive(Debug, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum InteractionInputEvent {
    CursorDownOnEntity(CursorDownOnEntity),
    CursorMovedOnComposition(CursorMovedOnComposition),
    CursorEnteredComposition(CursorEnteredComposition),
    CursorExitedComposition(CursorExitedComposition),
}

impl InputEvent for InteractionInputEvent {
    fn send_to_ecs(self, world: &mut World) {
        match self {
            InteractionInputEvent::CursorMovedOnComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorEnteredComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorExitedComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorDownOnEntity(event) => {
                world.send_event(event);
            } // ... other interaction events
        }
    }
}

// =============================================================================
// Cursor Events
// =============================================================================

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorMovedOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorEnteredComposition;

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorExitedComposition;

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnEntity {
    pub entity: Entity,
}