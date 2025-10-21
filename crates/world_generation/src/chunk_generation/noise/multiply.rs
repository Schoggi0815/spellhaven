use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Multiply<T1, T2> {
    source_1: T1,
    source_2: T2,
}

impl<T1, T2> Multiply<T1, T2> {
    pub fn new(source_1: T1, source_2: T2) -> Self {
        Self { source_1, source_2 }
    }
}

impl<T1, T2, TInput> NoiseFunction<NoiseResult, TInput> for Multiply<T1, T2>
where
    T1: NoiseFunction<NoiseResult, TInput>,
    T2: NoiseFunction<NoiseResult, TInput>,
    TInput: Copy,
{
    fn get(&self, input: TInput) -> NoiseResult {
        self.source_1.get(input) * self.source_2.get(input)
    }
}
