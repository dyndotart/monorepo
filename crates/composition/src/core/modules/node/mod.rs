use bevy_app::{App, Plugin, PostUpdate};

use self::systems::construct_path::{
    rectangle::construct_rectangle_path, text::construct_text_path,
};

pub mod components;
pub mod helper;
mod systems;
pub mod utils;

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        // Register systems
        app.add_systems(PostUpdate, (construct_rectangle_path, construct_text_path));
    }
}
