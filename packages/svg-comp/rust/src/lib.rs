mod bindgen;
pub mod events;
pub mod modules;

use bevy_app::App;
use dyn_comp_core::CompCorePlugin;
use dyn_comp_dtif::CompDtif;
use dyn_comp_interaction::CompInteractionPlugin;
use dyn_comp_svg_builder::{events::SvgBuilderOutputEvent, CompSvgBuilderPlugin};
use dyn_comp_types::events::InputEvent;
use events::{SvgCompInputEvent, SvgCompOutputEvent};
use modules::watch::CompWatchPlugin;
use std::sync::mpsc::{channel, Receiver};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct SvgCompHandle {
    app: App,
    svg_builder_output_event_receiver: Receiver<SvgBuilderOutputEvent>,
    output_event_receiver: Receiver<SvgCompOutputEvent>,
}

#[wasm_bindgen]
impl SvgCompHandle {
    pub fn create(js_dtif: JsValue, interactive: bool) -> Result<SvgCompHandle, JsValue> {
        let dtif: CompDtif = serde_wasm_bindgen::from_value(js_dtif)?;
        let mut app = App::new();

        log::info!("[create] Dtif {:#?}", dtif);

        let (svg_builder_output_event_sender, svg_builder_output_event_receiver) =
            channel::<SvgBuilderOutputEvent>();
        let (output_event_sender, output_event_receiver) = channel::<SvgCompOutputEvent>();

        // Register plugins
        app.add_plugins((
            CompCorePlugin { dtif },
            CompWatchPlugin {
                output_event_sender,
            },
            CompSvgBuilderPlugin {
                output_event_sender: svg_builder_output_event_sender,
            },
        ));
        if interactive {
            app.add_plugins(CompInteractionPlugin);
        }

        return Ok(Self {
            app,
            svg_builder_output_event_receiver,
            output_event_receiver,
        });
    }

    pub fn update(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<SvgCompInputEvent>, _> =
            serde_wasm_bindgen::from_value(js_input_events);

        log::info!("[update] Input Events {:#?}", maybe_input_events);

        // Emit input events in ECS world
        if let Ok(input_events) = maybe_input_events {
            for input_event in input_events {
                match input_event {
                    SvgCompInputEvent::Comp { event } => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                    SvgCompInputEvent::Interaction { event } => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                }
            }
        }

        // Advance the execution of the schedule by one cycle
        self.app.update();

        // Collect output events that were emitted during the last update cycle
        let mut output_events = Vec::new();
        while let Ok(event) = self.output_event_receiver.try_recv() {
            output_events.push(event);
        }
        while let Ok(event) = self.svg_builder_output_event_receiver.try_recv() {
            match event {
                SvgBuilderOutputEvent::ElementChanges(event) => {
                    output_events.push(SvgCompOutputEvent::ElementChanges(event))
                }
            }
        }

        log::info!("[update] Output Events {:#?}", output_events);

        return Ok(serde_wasm_bindgen::to_value(&output_events)?);
    }
}

// #[cfg(test)]
// mod tests {
//     use specta::{
//         export,
//         ts::{BigIntExportBehavior, ExportConfig},
//     };

//     #[test]
//     fn specta_works() {
//         export::ts_with_cfg(
//             "./bindings.ts",
//             "".into(),
//             &ExportConfig::default().bigint(BigIntExportBehavior::Number),
//         )
//         .unwrap();
//     }
// }
