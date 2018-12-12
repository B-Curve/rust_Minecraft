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

pub use glm::builtin as Geom;
pub use glm::ext as Ext;
