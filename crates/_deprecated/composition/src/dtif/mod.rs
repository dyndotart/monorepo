use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::modules::{
    composition::{
        events::CoreInputEvent,
        resources::{composition::ViewBox, font_cache::font::Font},
    },
    node::components::bundles::{NodeBundle, PaintBundle},
};

pub mod dtif_processor;

/// Represents the composition in which all nodes exist.
#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct DTIFComposition {
    /// The version of the composition type declaration, used internally.
    /// Defaults to the latest version.
    #[serde(default = "default_dtif_version")]
    pub version: String,

    /// The name of the composition.
    /// Example: 'My super cool composition'.
    pub name: String,

    /// The width of the composition, in units.
    pub width: f32,

    /// The height of the composition, in units.
    pub height: f32,

    #[serde(default)]
    pub view_box: Option<ViewBox>,

    /// The identifier of the root node in the composition.
    pub root_node_id: Entity,

    /// A mapping of node identifiers to their corresponding nodes within the composition.
    /// Note: Planned to directly use `Entity` as a key once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    pub nodes: HashMap<String, NodeBundle>,

    /// A mapping of paint identifiers to their corresponding paints within the composition.
    /// Note: Planned to directly use `Entity` as a key once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    pub paints: HashMap<String, PaintBundle>,

    /// A mapping of font identifiers to their corresponding font data within the composition.
    /// Note: Planned to directly use `u64` as a key once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub fonts: Option<HashMap<String, Font>>,

    /// Optional list of changes represented as core input events.
    /// This field is optional and defaults to `None` if not provided.
    #[serde(default)]
    pub changes: Option<Vec<CoreInputEvent>>,
}

fn default_dtif_version() -> String {
    String::from("1.0")
}
