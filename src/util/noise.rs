use noise::{NoiseFn, Fbm, MultiFractal};
use math::{Vec3i, Vec2i};
use world::constants::CHUNK_HEIGHT;

lazy_static! {
    static ref BLOCK_NOISE: Fbm = Fbm::new()
        .set_octaves(4)
        .set_frequency(0.07)
        .set_persistence(0.4);

    static ref HEIGHT_NOISE: Fbm = Fbm::new()
        .set_octaves(6)
        .set_frequency(0.0012)
        .set_persistence(0.35)
        .set_lacunarity(3.0);
}

pub fn noise_3i(p: Vec3i, threshold: usize) -> usize {
    let value = BLOCK_NOISE.get([
        p.x as f64 + 0.33, p.y as f64 + 0.11, p.z as f64 + 0.33
    ]).abs() * 2.0;
    (value * threshold as f64) as usize
}

pub fn height(world_x: i32, world_z: i32) -> i32 {
    (HEIGHT_NOISE.get([world_x as f64 + 1.33, world_z as f64 - 7.55]).abs()
        * 2.0 * CHUNK_HEIGHT as f64) as i32
}