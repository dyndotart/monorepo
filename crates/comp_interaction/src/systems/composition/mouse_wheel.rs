use crate::{
    events::MouseWheeledOnCompInputEvent,
    input::keyboard::{KeyCode, KeyCodeButtonInput},
};
use bevy_ecs::{
    event::EventReader,
    system::{Res, ResMut},
};
use dyn_comp_bundles::properties::Viewport;
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_utils::{properties::size::Size, units::abs::Abs};

static ZOOM_FACTOR: f32 = 0.9;

pub fn mouse_wheeled_on_comp_input_system(
    mut event_reader: EventReader<MouseWheeledOnCompInputEvent>,
    mut comp_res: ResMut<CompositionRes>,
    keyboard_input_res: Res<KeyCodeButtonInput>,
) {
    for event in event_reader.read() {
        let CompositionRes {
            size,
            viewport:
                Viewport {
                    physical_position,
                    physical_size,
                },
            ..
        } = comp_res.as_ref();
        let MouseWheeledOnCompInputEvent {
            position: cursor_position,
            delta,
        } = &event;

        let ctrl_key_pressed =
            keyboard_input_res.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
        let meta_key_pressed = keyboard_input_res.any_pressed([KeyCode::Meta]);

        if ctrl_key_pressed || meta_key_pressed {
            let scale_factor = if delta.y < 0.0 {
                1.0 / ZOOM_FACTOR
            } else {
                ZOOM_FACTOR
            };

            // Calculate relative cursor position within the composition
            let relative_cursor =
                (*cursor_position / size.to_vec2()) * physical_size.to_vec2() + *physical_position;

            let new_physical_size = Size::new(
                Abs::pt(physical_size.width() * scale_factor),
                Abs::pt(physical_size.height() * scale_factor),
            );
            let new_physical_position =
                relative_cursor - (*cursor_position / size.to_vec2()) * new_physical_size.to_vec2();

            // Update the composition's viewport
            comp_res.viewport.physical_position = new_physical_position;
            comp_res.viewport.physical_size = new_physical_size;
        } else {
            comp_res.viewport.physical_position += *delta;
        }
    }
}