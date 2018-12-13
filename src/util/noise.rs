use noise::{NoiseFn, Fbm, MultiFractal};
use math::Vec3i;

lazy_static! {
    static ref NOISE: Fbm = Fbm::new()
            .set_octaves(2)
            .set_frequency(0.028)
            .set_lacunarity(0.25)
            .set_persistence(0.765);
}

pub fn noise_3i(p: Vec3i, threshold: usize) -> usize {
    let value = NOISE.get([
        p.x as f64 + 0.33, p.y as f64 + 0.11, p.z as f64 + 0.33
    ]).abs() * 2.0;
    (value * threshold as f64) as usize
}