use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Power<T1> {
    source: T1,
    exponent: f64,
}

impl<T1> Power<T1> {
    pub fn new(source: T1, exponent: f64) -> Self {
        Self { source, exponent }
    }
}

impl<T1, TInput> NoiseFunction<NoiseResult, TInput> for Power<T1>
where
    T1: NoiseFunction<NoiseResult, TInput>,
    TInput: Copy,
{
    fn get(&self, input: TInput) -> NoiseResult {
        let noise = self.source.get(input);

        NoiseResult {
            value: noise.value.powf(self.exponent),
            derivative: [
                noise.derivative[0]
                    * (self.exponent * noise.value.powf(self.exponent - 1.)),
                noise.derivative[1]
                    * (self.exponent * noise.value.powf(self.exponent - 1.)),
            ],
        }
    }
}
