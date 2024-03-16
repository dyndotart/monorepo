use bevy_transform::components::Transform;
use glam::{EulerRot, Mat4, Quat, Vec3};

pub fn transform_to_z_rotation_rad(transform: &Transform) -> f32 {
    transform.rotation.to_euler(EulerRot::XYZ).2
}

// Adjusts Bevy's right-handed coordinate system transformation for SVG's left-handed system
// by mirroring the z-axis
// https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Positions
// https://bevy-cheatbook.github.io/fundamentals/coords.html
// https://www.mikechambers.com/blog/2022/10/29/understanding-the-2d-coordinate-system-in-bevy/
// https://bevy-cheatbook.github.io/fundamentals/coords.html
pub fn convert_rh_to_lh(mat4: Mat4) -> Mat4 {
    // Create a scaling matrix that mirrors the z-axis
    let mirror_z = Mat4::from_scale(Vec3::new(1.0, 1.0, -1.0));

    return mirror_z * mat4;
}

// https://math.stackexchange.com/questions/2093314/rotation-matrix-of-rotation-around-a-point-other-than-the-origin
pub fn rotate_around_point(transform: Mat4, angle_rad: f32, pivot_point: Vec3) -> Mat4 {
    let translate_to_pivot = Mat4::from_translation(pivot_point);
    let translate_to_origin = Mat4::from_translation(-pivot_point);
    let rotation = Mat4::from_quat(Quat::from_rotation_z(angle_rad));

    return transform * (translate_to_pivot * rotation * translate_to_origin);
}
