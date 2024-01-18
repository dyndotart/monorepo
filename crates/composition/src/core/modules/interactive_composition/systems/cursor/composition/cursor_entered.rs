use bevy_ecs::event::EventReader;
use log::info;

use crate::core::modules::interactive_composition::events::CursorEnteredComposition;

pub fn handle_cursor_entered_composition(mut event_reader: EventReader<CursorEnteredComposition>) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        info!("handle_cursor_entered_composition");
    }
}
