use util::math::{Vec3, Vec2, Mat4};
use world::block::block::Block;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2,
    pub normal: Vec3
}