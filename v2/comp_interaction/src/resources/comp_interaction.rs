use bevy_ecs::system::Resource;
use glam::Vec2;

#[derive(Resource, Debug, Default)]
pub struct CompInteractionRes {
    pub interaction_mode: InteractionMode,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionMode {
    /// Default canvas mode. Nothing is happening.
    #[default]
    None,
    /// When the user's pointer is pressed.
    Pressing { origin: Vec2, button: MouseButton },
    /// When the user is dragging.
    Dragging { current: Vec2 },
    /// When the user is moving selected nodes.
    Translating { origin: Vec2, current: Vec2 },
    /// When the user is resizing the selected nodes.
    Resizing {
        corner: u8,
        initial_bounds: XYWH,
        rotation_in_degrees: f32, // For cursor
    },
    /// When the user is rotating the selected nodes.
    Rotating {
        corner: u8,
        initial_rotation_in_radians: f32,
        rotation_in_degrees: f32, // For cursor
    },
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct XYWH {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub enum HandleSide {
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unkown,
}