pub mod events;
mod resources;
pub mod svg;
mod systems;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use resources::svg_context::SvgContextRes;
use systems::{
    apply::{
        apply_blend_mode_mixin_changes, apply_clip_content_changes, apply_drop_shadow_changes,
        apply_gradient_paint_changes, apply_image_asset_mixin_changes, apply_image_paint_changes,
        apply_node_children_changes, apply_node_styles_changes, apply_opacity_mixin_changes,
        apply_path_mixin_changes, apply_size_mixin_changes, apply_solid_paint_changes,
        apply_stroke_path_mixin_changes, apply_transform_changes, apply_visibility_mixin_changes,
    },
    prepare::{insert_node_svg_bundle, insert_style_svg_bundle, propagate_size_mixin_to_style},
};

pub struct ArbSvgBuilderPlugin {
    // TODO: Figure out why "rust-analyzer" doesn't check that "output_event_sender" is behind a feature
    // #[cfg(any(feature = "output_svg_element_changes", feature = "output_svg_string"))]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SvgBuilderOutputEvent>,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
// Currently, the challenge lies in managing the spawning (when absent)
// and modification of the SvgBundle component alongside its associated entity,
// due to the deferred execution nature of entity spawn commands within the ECS schedule.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ArbSvgBuilderSystemSet {
    Prepare,
    PreApply,
    Apply,
    Extract,
    Queue,
}

impl Plugin for ArbSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();

        // Configure system set
        app.configure_sets(
            PostUpdate,
            (
                ArbSvgBuilderSystemSet::Prepare,
                ArbSvgBuilderSystemSet::PreApply,
                ArbSvgBuilderSystemSet::Apply,
                ArbSvgBuilderSystemSet::Extract,
                ArbSvgBuilderSystemSet::Queue,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            PostUpdate,
            (
                propagate_size_mixin_to_style.in_set(ArbSvgBuilderSystemSet::Prepare),
                insert_node_svg_bundle.in_set(ArbSvgBuilderSystemSet::Prepare),
                insert_style_svg_bundle
                    .in_set(ArbSvgBuilderSystemSet::Prepare)
                    .after(insert_node_svg_bundle),
                apply_node_children_changes.in_set(ArbSvgBuilderSystemSet::PreApply),
                apply_node_styles_changes
                    .in_set(ArbSvgBuilderSystemSet::PreApply)
                    .after(apply_node_children_changes),
                apply_visibility_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_size_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_transform_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_opacity_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_blend_mode_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_clip_content_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_path_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_stroke_path_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_solid_paint_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_gradient_paint_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_image_paint_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_image_asset_mixin_changes.in_set(ArbSvgBuilderSystemSet::Apply),
                apply_drop_shadow_changes.in_set(ArbSvgBuilderSystemSet::Apply),
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
                PostUpdate,
                (
                    extract_node_bundles.in_set(ArbSvgBuilderSystemSet::Extract),
                    extract_style_bundles.in_set(ArbSvgBuilderSystemSet::Extract),
                    queue_svg_bundle_changes.in_set(ArbSvgBuilderSystemSet::Queue),
                ),
            );
        }
    }
}
