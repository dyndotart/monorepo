use crate::common::{Degree, Size, Viewport};
use bevy_ecs::{entity::Entity, event::Event, world::World};
use std::fmt::Debug;

pub trait InputEvent {
    fn send_into_ecs(self, world: &mut World);
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CompCoreInputEvent {
    EntityDeleted(EntityDeletedInputEvent),
    CompositionResized(CompositionResizedInputEvent),
    CompositionViewportChanged(CompositionViewportChangedInputEvent),
    EntityMoved(EntityMovedInputEvent),
    EntitySetPosition(EntitySetPositionInputEvent),
    EntitySetRotation(EntitySetRotationInputEvent),
}

impl InputEvent for CompCoreInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            CompCoreInputEvent::CompositionResized(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::CompositionViewportChanged(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::EntityMoved(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::EntitySetPosition(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::EntityDeleted(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::EntitySetRotation(event) => {
                world.send_event(event);
            }
        }
    }
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionResizedInputEvent {
    pub size: Size,
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionViewportChangedInputEvent {
    pub viewport: Viewport,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityMovedInputEvent {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntitySetPositionInputEvent {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityDeletedInputEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct EntitySetRotationInputEvent {
    pub entity: Entity,
    pub rotation_deg: Degree,
}