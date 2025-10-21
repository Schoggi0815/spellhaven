use crate::chunk_generation::{
    chunk_lod::ChunkLod,
    noise::{noise_function::NoiseFunction, noise_result::NoiseResult},
};

pub struct LodHeightAdjuster<T> {
    noise: T,
    lod: ChunkLod,
}

impl<T> LodHeightAdjuster<T> {
    pub const DEFAULT_LOD: ChunkLod = ChunkLod::Full;

    pub fn new(source: T, lod: ChunkLod) -> Self {
        Self {
            noise: source,
            lod: lod,
        }
    }

    pub fn set_lod(self, lod: ChunkLod) -> Self {
        Self { lod, ..self }
    }
}

impl<T> Default for LodHeightAdjuster<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            noise: Default::default(),
            lod: Self::DEFAULT_LOD,
        }
    }
}

impl<T> NoiseFunction<NoiseResult, [f64; 2]> for LodHeightAdjuster<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    fn get(&self, point: [f64; 2]) -> NoiseResult {
        let result = self.noise.get(point);
        let value = result.value * (1. / self.lod.multiplier_i32() as f64)
            + 1.
            + (10. / self.lod.multiplier_i32() as f64);

        NoiseResult {
            value,
            derivative: result.derivative,
        }
    }
}
