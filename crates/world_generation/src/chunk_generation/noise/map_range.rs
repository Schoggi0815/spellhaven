use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct MapRange<TBaseNoise> {
    noise: TBaseNoise,
    from_min: f64,
    from_max: f64,
    to_min: f64,
    to_max: f64,
}

impl<TBaseNoise> MapRange<TBaseNoise> {
    pub fn new(
        noise: TBaseNoise,
        from_min: f64,
        from_max: f64,
        to_min: f64,
        to_max: f64,
    ) -> Self {
        Self {
            noise,
            from_min,
            from_max,
            to_min,
            to_max,
        }
    }
}

impl<TBaseNoise, TInput> NoiseFunction<NoiseResult, TInput>
    for MapRange<TBaseNoise>
where
    TBaseNoise: NoiseFunction<NoiseResult, TInput>,
{
    fn get(&self, input: TInput) -> NoiseResult {
        let mut value = self.noise.get(input);

        value = value - self.from_min;

        let scale =
            (self.to_max - self.to_min) / (self.from_max - self.from_min);
        value = value * scale;
        value = value + self.to_min;
        value
    }
}
