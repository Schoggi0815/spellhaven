use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Negate<T> {
    source: T,
}

impl<T> Negate<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T, TInput> NoiseFunction<NoiseResult, TInput> for Negate<T>
where
    T: NoiseFunction<NoiseResult, TInput>,
    TInput: Copy,
{
    fn get(&self, input: TInput) -> NoiseResult {
        -self.source.get(input)
    }
}
