use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Max<T1, T2> {
    source_1: T1,
    source_2: T2,
}

impl<T1, T2> Max<T1, T2> {
    pub fn new(source_1: T1, source_2: T2) -> Self {
        Self { source_1, source_2 }
    }
}

impl<T1, T2, TInput> NoiseFunction<NoiseResult, TInput> for Max<T1, T2>
where
    T1: NoiseFunction<NoiseResult, TInput>,
    T2: NoiseFunction<NoiseResult, TInput>,
    TInput: Copy,
{
    fn get(&self, input: TInput) -> NoiseResult {
        let value_1 = self.source_1.get(input);
        let value_2 = self.source_2.get(input);
        NoiseResult {
            value: value_1.value.max(value_2.value),
            derivative: if value_1.value > value_2.value {
                value_1.derivative
            } else {
                value_2.derivative
            },
        }
    }
}
