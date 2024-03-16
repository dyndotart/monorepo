pub mod events;
mod resources;
pub mod svg;
mod systems;

use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use resources::svg_context::SvgContextRes;
use systems::{
    apply::{
        apply_blend_mode_mixin_changes, apply_gradient_paint_changes,
        apply_image_asset_mixin_changes, apply_image_paint_changes, apply_node_children_changes,
        apply_node_styles_changes, apply_opacity_mixin_changes, apply_path_mixin_changes,
        apply_size_mixin_changes, apply_solid_paint_changes, apply_stroke_path_mixin_changes,
        apply_transform_changes, apply_visibility_mixin_changes,
    },
    prepare::{insert_node_svg_bundle, insert_style_svg_bundle, sync_node_size_with_style},
};

pub struct CompSvgBuilderPlugin {
    #[cfg(any(feature = "output_svg_element_changes", feature = "output_svg_string"))]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SvgBuilderOutputEvent>,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
// Currently, the challenge lies in managing the spawning (when absent)
// and modification of the SvgBundle component alongside its associated entity,
// due to the deferred execution nature of entity spawn commands within the ECS schedule.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompSvgBuilderSystemSet {
    Prepare,
    PreApply,
    Apply,
    Extract,
    Queue,
}

impl Plugin for CompSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (
                CompSvgBuilderSystemSet::Prepare,
                CompSvgBuilderSystemSet::PreApply,
                CompSvgBuilderSystemSet::Apply,
                CompSvgBuilderSystemSet::Extract,
                CompSvgBuilderSystemSet::Queue,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            Last,
            (
                sync_node_size_with_style.in_set(CompSvgBuilderSystemSet::Prepare),
                insert_node_svg_bundle.in_set(CompSvgBuilderSystemSet::Prepare),
                insert_style_svg_bundle
                    .in_set(CompSvgBuilderSystemSet::Prepare)
                    .after(insert_node_svg_bundle),
                apply_node_children_changes.in_set(CompSvgBuilderSystemSet::PreApply),
                apply_node_styles_changes
                    .in_set(CompSvgBuilderSystemSet::PreApply)
                    .after(apply_node_children_changes),
                apply_visibility_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_size_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_transform_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_opacity_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_blend_mode_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_path_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_stroke_path_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_solid_paint_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_gradient_paint_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_image_paint_changes.in_set(CompSvgBuilderSystemSet::Apply),
                apply_image_asset_mixin_changes.in_set(CompSvgBuilderSystemSet::Apply),
            ),
        );

        #[cfg(feature = "output_svg_string")]
        {
            // TODO
        }

        #[cfg(feature = "output_svg_element_changes")]
        {
            use crate::resources::{
                changed_svg_bundles::ChangedSvgBundlesRes,
                output_event_sender::OutputEventSenderRes,
            };
            use crate::systems::{
                extract::{extract_node_bundles, extract_style_bundles},
                queue::queue_svg_bundle_changes,
            };

            // Register resources
            app.init_resource::<ChangedSvgBundlesRes>();
            app.insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));

            // Register systems
            app.add_systems(
                Last,
                (
                    extract_node_bundles.in_set(CompSvgBuilderSystemSet::Extract),
                    extract_style_bundles.in_set(CompSvgBuilderSystemSet::Extract),
                    queue_svg_bundle_changes.in_set(CompSvgBuilderSystemSet::Queue),
                ),
            );
        }
    }
}