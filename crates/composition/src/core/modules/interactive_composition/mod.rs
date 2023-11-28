use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};

use self::{
    events::{
        CursorDownOnComposition, CursorDownOnEntity, CursorDownOnResizeHandle,
        CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition,
        CursorUpOnComposition,
    },
    resources::InteractiveCompositionRes,
    systems::{
        handle_cursor_down_on_composition, handle_cursor_down_on_entity_event,
        handle_cursor_down_on_resize_handle, handle_cursor_entered_composition,
        handle_cursor_exited_composition, handle_cursor_moved_on_composition,
        handle_cursor_up_on_composition,
    },
};

pub mod events;
mod helper;
pub mod resources;
mod systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum InteractionSet {
    First,
    Initial,
    Continuous,
    Completion,
    Last,
}

pub struct InteractiveCompositionPlugin;

impl Plugin for InteractiveCompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();
        app.add_event::<CursorDownOnComposition>();
        app.add_event::<CursorUpOnComposition>();
        app.add_event::<CursorDownOnResizeHandle>();

        // Register resources
        app.world.init_resource::<InteractiveCompositionRes>();

        // Configure system sets
        app.configure_sets(
            PreUpdate,
            (
                InteractionSet::First,
                InteractionSet::Initial,
                InteractionSet::Continuous,
                InteractionSet::Completion,
                InteractionSet::Last,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_cursor_entered_composition.in_set(InteractionSet::First),
                handle_cursor_down_on_composition.in_set(InteractionSet::Initial),
                handle_cursor_down_on_entity_event
                    .in_set(InteractionSet::Initial)
                    .after(handle_cursor_down_on_composition),
                handle_cursor_down_on_resize_handle
                    .in_set(InteractionSet::Initial)
                    .after(handle_cursor_down_on_composition),
                handle_cursor_moved_on_composition.in_set(InteractionSet::Continuous),
                handle_cursor_up_on_composition.in_set(InteractionSet::Completion),
                handle_cursor_exited_composition.in_set(InteractionSet::Last),
            ),
        );
    }
}
