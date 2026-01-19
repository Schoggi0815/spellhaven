use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Abs<T> {
    source: T,
}

impl<T> Abs<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T, TInput> NoiseFunction<NoiseResult, TInput> for Abs<T>
where
    T: NoiseFunction<NoiseResult, TInput>,
    TInput: Copy,
{
    fn get(&self, input: TInput) -> NoiseResult {
        let value = self.source.get(input);

        NoiseResult {
            value: value.value.abs(),
            derivative: if value.value.is_sign_negative() {
                [-value.derivative[0], -value.derivative[1]]
            } else {
                value.derivative
            },
        }
    }
}
