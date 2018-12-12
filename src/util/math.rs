use glm;
pub use num_traits::One;
pub use num_traits::Zero;
use std::ops::{AddAssign, SubAssign};

use glm::{Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

pub type Vec2i = Vector2<i32>;
pub type Vec3i = Vector3<i32>;
pub type Vec4i = Vector4<i32>;

pub type Mat3 = glm::Matrix3<f32>;
pub type Mat4 = glm::Matrix4<f32>;

pub use glm::vec2 as vec2;
pub use glm::vec3 as vec3;
pub use glm::vec4 as vec4;

pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Mat4 {
    let mut p = Mat4::one();
    p[0][0] = 2.0 / (right - left);
    p[1][1] = 2.0 / (top - bottom);
    p[2][2] = -2.0 / (z_far - z_near);
    p[3][0] = -(right + left) / (right - left);
    p[3][1] = -(top + bottom) / (top - bottom);
    p[3][2] = -(z_far + z_near) / (z_far - z_near);

    p
}

pub use glm::builtin as Geom;
pub use glm::ext as Ext;
