use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_arb_bundles::components::{
    mixins::{PathMixin, SizeMixin, WindingRule},
    nodes::PolygonArbNode,
};
use std::f32::consts::PI;
use tiny_skia_path::PathBuilder;

pub fn outline_polygon(
    mut commands: Commands,
    query: Query<
        (Entity, &PolygonArbNode, &SizeMixin),
        Or<(Changed<PolygonArbNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, polygon, SizeMixin(size)) in query.iter() {
        if polygon.point_count < 3 {
            continue;
        }

        let radius = size.to_vec2() / 2.0;

        let mut path_builder = PathBuilder::new();

        for i in 0..polygon.point_count {
            let angle = 2.0 * PI / polygon.point_count as f32 * i as f32 - PI / 2.0;
            let (x, y) = (
                radius.x * angle.cos() + radius.x,
                radius.y * angle.sin() + radius.y,
            );

            // Move to the first vertex or line to subsequent vertices
            if i == 0 {
                path_builder.move_to(x, y);
            } else {
                path_builder.line_to(x, y);
            }
        }

        // Close the path to complete the polygon shape
        path_builder.close();

        // Insert or update the PathMixin component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
    }
}
