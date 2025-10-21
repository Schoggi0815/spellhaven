use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct ScalePoint<T> {
    source: T,
    scale: f64,
}

impl<T> ScalePoint<T> {
    pub fn new(source: T, scale: f64) -> Self {
        Self { source, scale }
    }
}

impl<T> NoiseFunction<NoiseResult, [f64; 2]> for ScalePoint<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    fn get(&self, input: [f64; 2]) -> NoiseResult {
        let result = self
            .source
            .get([input[0] * self.scale, input[1] * self.scale]);

        NoiseResult {
            value: result.value,
            derivative: [
                result.derivative[0] * self.scale,
                result.derivative[1] * self.scale,
            ],
        }
    }
}
