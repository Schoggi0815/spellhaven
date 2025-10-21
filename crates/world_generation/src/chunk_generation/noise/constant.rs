use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Constant {
    value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl<TInput> NoiseFunction<NoiseResult, TInput> for Constant {
    fn get(&self, _input: TInput) -> NoiseResult {
        NoiseResult {
            value: self.value,
            derivative: [0., 0.],
        }
    }
}
