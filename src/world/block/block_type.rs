use util::math::{Vec2, vec2, Zero};
use rand::{distributions::{Distribution, Standard}, Rng};

const IMG_WIDTH: f32 = 384.0;
const IMG_HEIGHT: f32 = 784.0;
const UV_WIDTH: f32 = 16.0 / IMG_WIDTH;
const UV_HEIGHT: f32 = 16.0 / IMG_HEIGHT;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BlockType {
    Air = 0,
    Dirt,
    Grass,
    DiamondOre,
    RedstoneOre,
    Pumpkin,
    JackOLantern,
    Torch
}

pub struct UvCoords {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
    pub d: Vec2
}

impl Default for UvCoords {
    fn default() -> Self {
        UvCoords {
            a: Zero::zero(),
            b: Zero::zero(),
            c: Zero::zero(),
            d: Zero::zero()
        }
    }
}

impl BlockType {
    pub fn tex_coords(location: Vec2, scale_x: f32, scale_y: f32) -> UvCoords {
        let x_max = (location.x + scale_x) * UV_WIDTH;
        let y_max = location.y * UV_HEIGHT;

        let x_min = location.x * UV_WIDTH;
        let y_min = (location.y + scale_y) * UV_HEIGHT;

        UvCoords {
            a: vec2(x_min, y_min),
            b: vec2(x_max, y_min),
            c: vec2(x_max, y_max),
            d: vec2(x_min, y_max)
        }
    }
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        use self::BlockType::*;
        match rng.gen_range(0, 3) {
            0 => Grass,
            1 => Dirt,
            2 => Torch,
            _ => Air
        }
    }
}