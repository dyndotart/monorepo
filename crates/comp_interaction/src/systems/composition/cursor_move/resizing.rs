use crate::{
    components::Selected,
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{HandleSide, XYWH},
    utils::{rotate_point, transform_point_to_viewport},
};
use bevy_ecs::{query::With, system::Query};
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_bundles::{
    components::mixins::SizeMixin,
    utils::{get_parent_global_transfrom, global_to_local_point3, transform_to_z_rotation_rad},
};
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_utils::units::abs::Abs;
use glam::Vec2;

pub fn handle_resizing(
    comp_res: &CompositionRes,
    selected_nodes_query: &mut Query<
        (&mut Transform, &mut SizeMixin, Option<&Parent>),
        With<Selected>,
    >,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnCompInputEvent,
    corner: u8,
    initial_bounds: &XYWH,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let global_cursor_position = transform_point_to_viewport(comp_res, cursor_position, true);

    for (mut transform, mut size_mixin, maybe_parent) in selected_nodes_query.iter_mut() {
        let SizeMixin(size) = size_mixin.as_mut();
        let maybe_parent_global_transform =
            get_parent_global_transfrom(maybe_parent, global_transform_query);
        let local_cursor_position = global_to_local_point3(
            global_cursor_position.extend(0.0),
            maybe_parent_global_transform,
        )
        .truncate();
        let local_initial_bounds = XYWH {
            position: global_to_local_point3(
                initial_bounds.position.extend(0.0),
                maybe_parent_global_transform,
            )
            .truncate(),
            size: initial_bounds.size,
        };

        let new_bounds = resize_bounds(
            &local_initial_bounds,
            corner,
            &local_cursor_position,
            -transform_to_z_rotation_rad(&transform),
        );

        transform.translation.x = new_bounds.position.x;
        transform.translation.y = new_bounds.position.y;
        *size = new_bounds.size;
    }
}

pub fn resize_bounds(bounds: &XYWH, corner: u8, cursor_point: &Vec2, angle_rad: f32) -> XYWH {
    let mut result = *bounds;
    let pivot = bounds.position;

    // Calculate the unrotated position of the cursor
    let unrotated_cursor_point = rotate_point(cursor_point, &pivot, angle_rad);

    // Adjust the bounds based on the unrotated cursor position
    if (corner & HandleSide::Left as u8) == HandleSide::Left as u8 {
        result.position.x = unrotated_cursor_point
            .x
            .min(bounds.position.x + bounds.size.width());
        result.size.width =
            Abs::pt((bounds.position.x + bounds.size.width() - unrotated_cursor_point.x).abs());
    }
    if (corner & HandleSide::Right as u8) == HandleSide::Right as u8 {
        result.position.x = unrotated_cursor_point.x.min(bounds.position.x);
        result.size.width = Abs::pt((unrotated_cursor_point.x - bounds.position.x).abs());
    }
    if (corner & HandleSide::Top as u8) == HandleSide::Top as u8 {
        result.position.y = unrotated_cursor_point
            .y
            .min(bounds.position.y + bounds.size.height() as f32);
        result.size.height =
            Abs::pt((bounds.position.y + bounds.size.height() - unrotated_cursor_point.y).abs());
    }
    if (corner & HandleSide::Bottom as u8) == HandleSide::Bottom as u8 {
        result.position.y = unrotated_cursor_point.y.min(bounds.position.y);
        result.size.height = Abs::pt((unrotated_cursor_point.y - bounds.position.y).abs());
    }

    // Rotate the bounds back to the original angle
    result.position = rotate_point(&result.position, &pivot, -angle_rad);

    return result;
}
