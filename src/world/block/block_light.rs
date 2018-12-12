use math::{Vec3, Zero};
use world::block::block_type::BlockType;

pub struct BlockLight {
    pub color: Vec3,
    pub strength: f32,
    pub position: Vec3,
    pub block_type: BlockType
}

impl Default for BlockLight {
    fn default() -> Self {
        BlockLight {
            color: Zero::zero(),
            strength: 0.0,
            position: Zero::zero(),
            block_type: BlockType::Air
        }
    }
}